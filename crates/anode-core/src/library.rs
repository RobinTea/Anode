use std::path::{Path, PathBuf};

use chrono::Utc;
use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::book::BookService;
use crate::models::{AppConfig, BookMeta, BookSummary};
use crate::paths::{self, atomic_write, default_library_path, library_root};
use crate::schema::init_library_db;
use crate::Result;

pub struct Library {
    pub root: PathBuf,
    conn: Connection,
}

impl Library {
    pub fn open(config: &AppConfig) -> Result<Self> {
        let root = library_root(config)
            .unwrap_or_else(default_library_path);
        Self::open_at(root)
    }

    pub fn open_at(root: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&root)?;
        std::fs::create_dir_all(root.join("books"))?;

        let db_path = root.join("library.db");
        let conn = Connection::open(&db_path)?;
        init_library_db(&conn)?;

        Ok(Self { root, conn })
    }

    pub fn path(&self) -> &Path {
        &self.root
    }

    pub fn list_books(&self) -> Result<Vec<BookSummary>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, author, updated_at FROM books ORDER BY updated_at DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })?;

        let mut out = Vec::new();
        for row in rows {
            let (id, title, author, updated_at) = row?;
            let id = Uuid::parse_str(&id).map_err(|e| crate::AnodeError::msg(e.to_string()))?;
            let updated_at = updated_at
                .parse()
                .unwrap_or_else(|_| Utc::now());

            let (write_page_count, total_words) =
                BookService::stats_for_book(&self.root, id)?;

            out.push(BookSummary {
                id,
                title,
                author,
                updated_at,
                write_page_count,
                total_words,
            });
        }
        Ok(out)
    }

    pub fn create_book(&self, title: &str) -> Result<BookMeta> {
        let book = BookService::create(&self.root, title)?;
        self.conn.execute(
            "INSERT INTO books (id, title, author, updated_at) VALUES (?1, ?2, ?3, ?4)",
            params![
                book.id.to_string(),
                book.title,
                book.author,
                book.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(book)
    }

    pub fn delete_book(&self, book_id: Uuid) -> Result<()> {
        let dir = paths::book_dir(&self.root, book_id);
        if dir.exists() {
            std::fs::remove_dir_all(&dir)?;
        }
        self.conn
            .execute("DELETE FROM books WHERE id = ?1", params![book_id.to_string()])?;
        Ok(())
    }

    pub fn touch_book(&self, book_id: Uuid) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "UPDATE books SET updated_at = ?1 WHERE id = ?2",
            params![now, book_id.to_string()],
        )?;
        Ok(())
    }

    pub fn register_existing_book(&self, meta: &BookMeta) -> Result<()> {
        self.conn.execute(
            r#"INSERT INTO books (id, title, author, updated_at) VALUES (?1, ?2, ?3, ?4)
               ON CONFLICT(id) DO UPDATE SET title = excluded.title, author = excluded.author, updated_at = excluded.updated_at"#,
            params![
                meta.id.to_string(),
                meta.title,
                meta.author,
                meta.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }
}

pub fn init_library_at(path: PathBuf) -> Result<Library> {
    std::fs::create_dir_all(&path)?;
    let mut config = paths::load_app_config()?;
    config.library_path = Some(path.to_string_lossy().into_owned());
    config.first_run_complete = true;
    paths::save_app_config(&config)?;
    Library::open_at(path)
}

pub fn is_first_run() -> Result<bool> {
    let config = paths::load_app_config()?;
    Ok(!config.first_run_complete || config.library_path.is_none())
}

pub fn get_config() -> Result<AppConfig> {
    paths::load_app_config()
}

pub fn set_library_path(path: PathBuf) -> Result<AppConfig> {
    let mut config = paths::load_app_config()?;
    config.library_path = Some(path.to_string_lossy().into_owned());
    config.first_run_complete = true;
    paths::save_app_config(&config)?;
    Ok(config)
}

pub fn write_lock_marker(library: &Path) -> Result<()> {
    let lock = library.join(".anode-lock");
    let pid = std::process::id();
    atomic_write(&lock, format!("{pid}\n").as_bytes())
}
