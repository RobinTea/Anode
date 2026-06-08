use std::path::Path;

use chrono::Utc;
use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::models::{CompileOrderEntry, PageBody, PageKind, PageMeta};
use crate::paths::{self, atomic_write, page_body_path, page_meta_path, snapshot_dir};
use crate::Result;

pub struct PageService;

impl PageService {
    pub fn create(
        library: &Path,
        book_id: Uuid,
        kind: PageKind,
        class: &str,
        title: &str,
    ) -> Result<PageMeta> {
        let dir = paths::book_dir(library, book_id);
        let conn = crate::book::BookService::open_db(library, book_id)?;
        let sort_key: i64 = conn.query_row(
            "SELECT COALESCE(MAX(sort_key), -1) + 1 FROM pages",
            [],
            |r| r.get(0),
        )?;
        Self::create_with_conn(&conn, &dir, kind, class, title, sort_key)
    }

    pub fn create_with_conn(
        conn: &Connection,
        book_dir: &Path,
        kind: PageKind,
        class: &str,
        title: &str,
        sort_key: i64,
    ) -> Result<PageMeta> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let kind_str = kind_to_str(kind);
        let meta = PageMeta {
            id,
            kind,
            class: class.to_string(),
            title: title.to_string(),
            sort_key,
            status: "draft".into(),
            word_count: 0,
            updated_at: now,
        };

        std::fs::create_dir_all(book_dir.join("pages"))?;

        let meta_json = serde_json::to_string_pretty(&meta)?;
        atomic_write(&page_meta_path(book_dir, id), meta_json.as_bytes())?;

        let body = PageBody::default();
        Self::write_body(book_dir, id, &body)?;

        conn.execute(
            r#"INSERT INTO pages (id, kind, class, title, sort_key, status, word_count, updated_at)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"#,
            params![
                id.to_string(),
                kind_str,
                meta.class,
                meta.title,
                meta.sort_key,
                meta.status,
                meta.word_count,
                meta.updated_at.to_rfc3339(),
            ],
        )?;

        if kind == PageKind::Write {
            let pos: i64 = conn.query_row(
                "SELECT COALESCE(MAX(position), -1) + 1 FROM compile_order",
                [],
                |r| r.get(0),
            )?;
            conn.execute(
                "INSERT OR REPLACE INTO compile_order (page_id, position, included) VALUES (?1, ?2, 1)",
                params![id.to_string(), pos],
            )?;
        }

        Ok(meta)
    }

    pub fn list(library: &Path, book_id: Uuid) -> Result<Vec<PageMeta>> {
        let conn = crate::book::BookService::open_db(library, book_id)?;
        let mut stmt = conn.prepare(
            "SELECT id, kind, class, title, sort_key, status, word_count, updated_at FROM pages ORDER BY sort_key ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, i64>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, u64>(6)?,
                row.get::<_, String>(7)?,
            ))
        })?;

        let mut out = Vec::new();
        for row in rows {
            let (id, kind, class, title, sort_key, status, word_count, updated_at) = row?;
            out.push(PageMeta {
                id: Uuid::parse_str(&id).map_err(|e| crate::AnodeError::msg(e.to_string()))?,
                kind: str_to_kind(&kind),
                class,
                title,
                sort_key,
                status,
                word_count,
                updated_at: updated_at.parse().unwrap_or_else(|_| Utc::now()),
            });
        }
        Ok(out)
    }

    pub fn load_body(library: &Path, book_id: Uuid, page_id: Uuid) -> Result<PageBody> {
        let path = page_body_path(&paths::book_dir(library, book_id), page_id);
        if !path.exists() {
            return Ok(PageBody::default());
        }
        let data = std::fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&data)?)
    }

    pub fn save_body(
        library: &Path,
        book_id: Uuid,
        page_id: Uuid,
        body: &PageBody,
    ) -> Result<()> {
        let book_dir = paths::book_dir(library, book_id);
        Self::write_body(&book_dir, page_id, body)?;

        let conn = crate::book::BookService::open_db(library, book_id)?;
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE pages SET word_count = ?1, updated_at = ?2 WHERE id = ?3",
            params![body.word_count, now, page_id.to_string()],
        )?;

        Self::maybe_snapshot(&book_dir, page_id, body)?;
        Ok(())
    }

    pub fn compile_order(library: &Path, book_id: Uuid) -> Result<Vec<CompileOrderEntry>> {
        let conn = crate::book::BookService::open_db(library, book_id)?;
        let mut stmt = conn.prepare(
            r#"SELECT c.page_id, c.position, c.included, p.title
               FROM compile_order c
               JOIN pages p ON p.id = c.page_id
               WHERE p.kind = 'write'
               ORDER BY c.position ASC"#,
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, String>(3)?,
            ))
        })?;

        let mut out = Vec::new();
        for row in rows {
            let (page_id, position, included, title) = row?;
            let page_id = Uuid::parse_str(&page_id).map_err(|e| crate::AnodeError::msg(e.to_string()))?;
            out.push(CompileOrderEntry {
                page_id,
                position,
                included: included != 0,
                title,
            });
        }
        Ok(out)
    }

    pub fn update_compile_order(
        library: &Path,
        book_id: Uuid,
        entries: &[CompileOrderEntry],
    ) -> Result<()> {
        let conn = crate::book::BookService::open_db(library, book_id)?;
        for e in entries {
            conn.execute(
                "INSERT OR REPLACE INTO compile_order (page_id, position, included) VALUES (?1, ?2, ?3)",
                params![e.page_id.to_string(), e.position, e.included as i64],
            )?;
        }
        Ok(())
    }

    pub fn list_snapshots(book_dir: &Path, page_id: Uuid) -> Result<Vec<SnapshotInfo>> {
        let dir = snapshot_dir(book_dir, page_id);
        if !dir.exists() {
            return Ok(Vec::new());
        }
        let mut snaps: Vec<_> = std::fs::read_dir(&dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|x| x == "json"))
            .collect();
        snaps.sort_by_key(|e| e.file_name());
        snaps.reverse();

        let mut result = Vec::new();
        for entry in snaps {
            if let Ok(metadata) = entry.metadata() {
                if let Ok(name) = entry.file_name().into_string() {
                    result.push(SnapshotInfo {
                        filename: name,
                        timestamp: chrono::DateTime::parse_from_rfc3339(
                            &format!("{}Z", &name.replace(".body.json", "")),
                        )
                        .ok()
                        .map(|dt| dt.with_timezone(&Utc).to_rfc3339())
                        .unwrap_or_default(),
                        size_bytes: metadata.len(),
                    });
                }
            }
        }
        Ok(result)
    }

    pub fn load_snapshot(book_dir: &Path, page_id: Uuid, filename: &str) -> Result<PageBody> {
        let path = snapshot_dir(book_dir, page_id).join(filename);
        if !path.exists() {
            return Err(crate::AnodeError::msg("Snapshot not found"));
        }
        let data = std::fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&data)?)
    }

    pub fn restore_snapshot(
        library: &Path,
        book_id: Uuid,
        page_id: Uuid,
        filename: &str,
    ) -> Result<()> {
        let book_dir = paths::book_dir(library, book_id);
        let body = Self::load_snapshot(&book_dir, page_id, filename)?;
        Self::save_body(library, book_id, page_id, &body)?;
        Ok(())
    }

    fn write_body(book_dir: &Path, page_id: Uuid, body: &PageBody) -> Result<()> {
        let data = serde_json::to_string_pretty(body)?;
        atomic_write(&page_body_path(book_dir, page_id), data.as_bytes())
    }

    fn maybe_snapshot(book_dir: &Path, page_id: Uuid, body: &PageBody) -> Result<()> {
        let dir = snapshot_dir(book_dir, page_id);
        std::fs::create_dir_all(&dir)?;
        let name = format!("{}.body.json", Utc::now().format("%Y%m%dT%H%M%SZ"));
        let path = dir.join(name);
        let data = serde_json::to_string_pretty(body)?;
        atomic_write(&path, data.as_bytes())?;

        // Keep last 10 snapshots per page
        let mut snaps: Vec<_> = std::fs::read_dir(&dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|x| x == "json"))
            .collect();
        snaps.sort_by_key(|e| e.file_name());
        while snaps.len() > 10 {
            if let Some(old) = snaps.first() {
                let _ = std::fs::remove_file(old.path());
            }
            snaps.remove(0);
        }
        Ok(())
    }
}

pub fn kind_to_str(kind: PageKind) -> &'static str {
    match kind {
        PageKind::Plan => "plan",
        PageKind::Write => "write",
        PageKind::Read => "read",
    }
}

pub fn str_to_kind(s: &str) -> PageKind {
    match s {
        "plan" => PageKind::Plan,
        "read" => PageKind::Read,
        _ => PageKind::Write,
    }
}

pub fn count_words_from_doc(doc: &serde_json::Value) -> u64 {
    fn walk(node: &serde_json::Value, buf: &mut String) {
        match node {
            serde_json::Value::Object(map) => {
                if let Some(serde_json::Value::String(t)) = map.get("text") {
                    buf.push_str(t);
                    buf.push(' ');
                }
                for v in map.values() {
                    walk(v, buf);
                }
            }
            serde_json::Value::Array(arr) => {
                for v in arr {
                    walk(v, buf);
                }
            }
            _ => {}
        }
    }
    let mut text = String::new();
    walk(doc, &mut text);
    text.split_whitespace().count() as u64
}
