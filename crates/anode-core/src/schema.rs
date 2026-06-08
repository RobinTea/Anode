use rusqlite::Connection;

use crate::Result;

pub fn init_library_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS books (
            id TEXT PRIMARY KEY NOT NULL,
            title TEXT NOT NULL,
            author TEXT NOT NULL DEFAULT '',
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS daily_words (
            date TEXT PRIMARY KEY NOT NULL,
            word_count INTEGER NOT NULL DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS todos (
            id TEXT PRIMARY KEY NOT NULL,
            text TEXT NOT NULL,
            done INTEGER NOT NULL DEFAULT 0,
            sort_key INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL
        );
        "#,
    )?;
    Ok(())
}

pub fn init_book_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS pages (
            id TEXT PRIMARY KEY NOT NULL,
            kind TEXT NOT NULL,
            class TEXT NOT NULL DEFAULT '',
            title TEXT NOT NULL,
            sort_key INTEGER NOT NULL DEFAULT 0,
            status TEXT NOT NULL DEFAULT 'draft',
            word_count INTEGER NOT NULL DEFAULT 0,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS compile_order (
            page_id TEXT PRIMARY KEY NOT NULL,
            position INTEGER NOT NULL,
            included INTEGER NOT NULL DEFAULT 1
        );

        CREATE TABLE IF NOT EXISTS character_relations (
            id TEXT PRIMARY KEY NOT NULL,
            from_id TEXT NOT NULL,
            to_id TEXT NOT NULL,
            label TEXT NOT NULL DEFAULT ''
        );
        "#,
    )?;
    Ok(())
}
