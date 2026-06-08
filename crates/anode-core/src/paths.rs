use std::path::{Path, PathBuf};

use crate::models::AppConfig;

pub fn app_config_dir() -> PathBuf {
    if let Ok(p) = std::env::var("ANODE_CONFIG_DIR") {
        return PathBuf::from(p);
    }
    dirs_path()
}

fn dirs_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        if let Ok(appdata) = std::env::var("APPDATA") {
            return PathBuf::from(appdata).join("Anode");
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home).join(".config").join("anode");
        }
    }
    PathBuf::from(".anode-config")
}

pub fn app_config_path() -> PathBuf {
    app_config_dir().join("config.json")
}

pub fn default_library_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        if let Ok(userprofile) = std::env::var("USERPROFILE") {
            return PathBuf::from(userprofile)
                .join("Documents")
                .join("Anode");
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home).join("Documents").join("Anode");
        }
    }
    PathBuf::from("Anode")
}

pub fn load_app_config() -> crate::Result<AppConfig> {
    let path = app_config_path();
    if !path.exists() {
        return Ok(AppConfig::default());
    }
    let data = std::fs::read_to_string(&path)?;
    Ok(serde_json::from_str(&data)?)
}

pub fn save_app_config(config: &AppConfig) -> crate::Result<()> {
    let path = app_config_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let data = serde_json::to_string_pretty(config)?;
    atomic_write(&path, data.as_bytes())
}

pub fn library_root(config: &AppConfig) -> Option<PathBuf> {
    config
        .library_path
        .as_ref()
        .map(PathBuf::from)
        .filter(|p| p.exists() || p.parent().is_some())
}

pub fn book_dir(library: &Path, book_id: uuid::Uuid) -> PathBuf {
    library.join("books").join(book_id.to_string())
}

pub fn page_body_path(book_dir: &Path, page_id: uuid::Uuid) -> PathBuf {
    book_dir
        .join("pages")
        .join(format!("{page_id}.body.json"))
}

pub fn page_meta_path(book_dir: &Path, page_id: uuid::Uuid) -> PathBuf {
    book_dir
        .join("pages")
        .join(format!("{page_id}.meta.json"))
}

pub fn snapshot_dir(book_dir: &Path, page_id: uuid::Uuid) -> PathBuf {
    book_dir.join("snapshots").join(page_id.to_string())
}

/// Write to `path.tmp` then rename for crash safety.
pub fn atomic_write(path: &Path, bytes: &[u8]) -> crate::Result<()> {
    let tmp = path.with_extension("tmp");
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&tmp, bytes)?;
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    std::fs::rename(&tmp, path)?;
    Ok(())
}
