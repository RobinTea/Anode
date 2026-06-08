use std::path::{Path, PathBuf};

use chrono::Utc;
use rusqlite::Connection;
use uuid::Uuid;

use crate::models::{BookMeta, PageKind};
use crate::page::PageService;
use crate::paths::{self, atomic_write};
use crate::schema::init_book_db;
use crate::Result;

pub struct BookService;

impl BookService {
    pub fn create(library: &Path, title: &str) -> Result<BookMeta> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let meta = BookMeta {
            id,
            title: title.to_string(),
            author: String::new(),
            genre: String::new(),
            synopsis: String::new(),
            created_at: now,
            updated_at: now,
        };

        let dir = paths::book_dir(library, id);
        std::fs::create_dir_all(dir.join("pages"))?;
        std::fs::create_dir_all(dir.join("snapshots"))?;

        let meta_json = serde_json::to_string_pretty(&meta)?;
        atomic_write(&dir.join("book.meta.json"), meta_json.as_bytes())?;

        let conn = Connection::open(dir.join("book.db"))?;
        init_book_db(&conn)?;

        PageService::create_with_conn(
            &conn,
            &dir,
            PageKind::Write,
            "chapter",
            "Chapter 1",
            0,
        )?;

        Ok(meta)
    }

    pub fn load_meta(library: &Path, book_id: Uuid) -> Result<BookMeta> {
        let path = paths::book_dir(library, book_id).join("book.meta.json");
        let data = std::fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&data)?)
    }

    pub fn save_meta(library: &Path, meta: &BookMeta) -> Result<()> {
        let path = paths::book_dir(library, meta.id).join("book.meta.json");
        let data = serde_json::to_string_pretty(meta)?;
        atomic_write(&path, data.as_bytes())
    }

    pub fn book_db_path(library: &Path, book_id: Uuid) -> PathBuf {
        paths::book_dir(library, book_id).join("book.db")
    }

    pub fn open_db(library: &Path, book_id: Uuid) -> Result<Connection> {
        let path = Self::book_db_path(library, book_id);
        let conn = Connection::open(&path)?;
        init_book_db(&conn)?;
        Ok(conn)
    }

    pub fn stats_for_book(library: &Path, book_id: Uuid) -> Result<(u32, u64)> {
        let dir = paths::book_dir(library, book_id);
        if !dir.exists() {
            return Ok((0, 0));
        }
        let conn = Self::open_db(library, book_id)?;
        let write_pages: u32 = conn.query_row(
            "SELECT COUNT(*) FROM pages WHERE kind = 'write'",
            [],
            |r| r.get(0),
        )?;
        let words: u64 = conn
            .query_row(
                "SELECT COALESCE(SUM(word_count), 0) FROM pages WHERE kind = 'write'",
                [],
                |r| r.get(0),
            )
            .unwrap_or(0);
        Ok((write_pages, words))
    }

    pub fn rebuild_search_index(_library: &Path, _book_id: Uuid) -> Result<()> {
        // TODO: Implement tantivy search index rebuild
        Ok(())
    }
}
