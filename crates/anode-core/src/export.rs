use sha2::{Sha256, Digest};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use uuid::Uuid;

use crate::{Result, AnodeError};
use crate::paths::book_dir;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ExportManifest {
    pub format_version: u32,
    pub book_id: String,
    pub created_at: String,
    pub include_snapshots: bool,
    pub files: Vec<ExportFile>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ExportFile {
    pub path: String,
    pub offset: u64,
    pub compressed_size: u64,
    pub uncompressed_size: u64,
    pub sha256: String,
}

const MAGIC_HEADER: &[u8; 4] = b"ANOD";
const FORMAT_VERSION: u32 = 1;

pub fn export_book(
    library: &Path,
    book_id: Uuid,
    include_snapshots: bool,
    output_path: &Path,
) -> Result<()> {
    let book_dir = book_dir(library, book_id);
    if !book_dir.exists() {
        return Err(AnodeError::msg("Book directory not found"));
    }

    let mut payload = Vec::new();
    let mut files = Vec::new();
    let mut current_offset: u64 = 0;

    // Collect files to export
    let mut file_paths = collect_export_files(&book_dir, include_snapshots)?;
    file_paths.sort();

    // Compress and collect file data
    for file_path in file_paths {
        let full_path = book_dir.join(&file_path);
        if !full_path.exists() {
            continue;
        }

        let data = fs::read(&full_path)?;
        let compressed = zstd::encode_all(data.as_slice(), 5)?;
        
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let hash = format!("{:x}", hasher.finalize());

        let uncompressed_size = data.len() as u64;
        let compressed_size = compressed.len() as u64;

        files.push(ExportFile {
            path: file_path,
            offset: current_offset,
            compressed_size,
            uncompressed_size,
            sha256: hash,
        });

        payload.extend_from_slice(&compressed);
        current_offset += compressed_size;
    }

    // Create manifest
    let manifest = ExportManifest {
        format_version: FORMAT_VERSION,
        book_id: book_id.to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        include_snapshots,
        files,
    };

    let manifest_json = serde_json::to_vec(&manifest)?;
    let manifest_compressed = zstd::encode_all(manifest_json.as_slice(), 5)?;

    // Write file: HEADER + PAYLOAD + MANIFEST
    let mut output = Vec::new();
    output.extend_from_slice(MAGIC_HEADER);
    output.extend_from_slice(&FORMAT_VERSION.to_le_bytes());
    output.extend_from_slice(&(current_offset as u64).to_le_bytes());
    output.extend_from_slice(&(manifest_compressed.len() as u64).to_le_bytes());
    output.extend_from_slice(&payload);
    output.extend_from_slice(&manifest_compressed);

    fs::write(output_path, output)?;
    Ok(())
}

pub fn import_book(
    library: &Path,
    anode_path: &Path,
) -> Result<Uuid> {
    let data = fs::read(anode_path)?;
    
    if data.len() < 20 {
        return Err(AnodeError::msg("Invalid .anode file: too small"));
    }

    // Parse header
    if &data[0..4] != MAGIC_HEADER {
        return Err(AnodeError::msg("Invalid .anode file: bad magic header"));
    }

    let format_version = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
    if format_version != FORMAT_VERSION {
        return Err(AnodeError::msg(format!("Unsupported format version: {}", format_version)));
    }

    let payload_size = u64::from_le_bytes([
        data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15],
    ]) as usize;
    let manifest_size = u64::from_le_bytes([
        data[16], data[17], data[18], data[19], data[20], data[21], data[22], data[23],
    ]) as usize;

    let manifest_offset = 24 + payload_size;
    if manifest_offset + manifest_size > data.len() {
        return Err(AnodeError::msg("Invalid .anode file: corrupted manifest"));
    }

    // Decompress manifest
    let manifest_compressed = &data[manifest_offset..manifest_offset + manifest_size];
    let manifest_json = zstd::decode_all(manifest_compressed)?;
    let manifest: ExportManifest = serde_json::from_slice(&manifest_json)?;

    // Create book directory
    let book_id = Uuid::parse_str(&manifest.book_id)
        .map_err(|e| AnodeError::msg(format!("Invalid book ID: {}", e)))?;
    let book_path = book_dir(library, book_id);
    fs::create_dir_all(&book_path)?;

    // Extract files
    let payload = &data[24..24 + payload_size];
    for file_info in &manifest.files {
        let compressed_data = &payload[file_info.offset as usize..
            (file_info.offset + file_info.compressed_size) as usize];
        let decompressed = zstd::decode_all(compressed_data)?;

        // Verify hash
        let mut hasher = Sha256::new();
        hasher.update(&decompressed);
        let hash = format!("{:x}", hasher.finalize());
        if hash != file_info.sha256 {
            return Err(AnodeError::msg(format!(
                "Hash mismatch for file: {}",
                file_info.path
            )));
        }

        // Write file
        let file_path = book_path.join(&file_info.path);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&file_path, decompressed)?;
    }

    // Rebuild search index
    crate::book::BookService::rebuild_search_index(library, book_id)?;

    Ok(book_id)
}

fn collect_export_files(book_dir: &Path, include_snapshots: bool) -> Result<Vec<String>> {
    let mut files = Vec::new();

    // Add metadata files
    if book_dir.join("book.meta.json").exists() {
        files.push("book.meta.json".to_string());
    }
    if book_dir.join("book.db").exists() {
        files.push("book.db".to_string());
    }

    // Add pages
    let pages_dir = book_dir.join("pages");
    if pages_dir.exists() {
        for entry in fs::read_dir(&pages_dir)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(file_name) = path.file_name() {
                if let Some(name_str) = file_name.to_str() {
                    files.push(format!("pages/{}", name_str));
                }
            }
        }
    }

    // Add snapshots if included
    if include_snapshots {
        let snapshots_dir = book_dir.join("snapshots");
        if snapshots_dir.exists() {
            for entry in fs::read_dir(&snapshots_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    for snap_entry in fs::read_dir(&path)? {
                        let snap_entry = snap_entry?;
                        let snap_path = snap_entry.path();
                        if let Some(rel_path) = snap_path.strip_prefix(&book_dir).ok() {
                            if let Some(rel_str) = rel_path.to_str() {
                                files.push(rel_str.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(files)
}
