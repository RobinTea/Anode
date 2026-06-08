use std::path::Path;

use chrono::Utc;
use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::models::TodoItem;
use crate::paths::library_db;
use crate::{AnodeError, Result};

pub struct TodoService;

impl TodoService {
    pub fn list(library: &Path) -> Result<Vec<TodoItem>> {
        let conn = Connection::open(library_db(library))?;
        let mut stmt = conn.prepare(
            "SELECT id, text, done, sort_key, created_at FROM todos ORDER BY sort_key ASC",
        )?;
        
        let mut items = Vec::new();
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let id: String = row.get(0)?;
            let text: String = row.get(1)?;
            let done: i32 = row.get(2)?;
            let sort_key: i64 = row.get(3)?;
            let created_at: String = row.get(4)?;
            
            let id_uuid = Uuid::parse_str(&id)
                .map_err(|e| AnodeError::msg(format!("Invalid UUID: {}", e)))?;
            
            items.push(TodoItem {
                id: id_uuid,
                text,
                done: done != 0,
                sort_key,
                created_at,
            });
        }
        Ok(items)
    }

    pub fn create(library: &Path, text: &str) -> Result<TodoItem> {
        let conn = Connection::open(library_db(library))?;
        let id = Uuid::new_v4();
        let now = Utc::now().to_rfc3339();
        let sort_key = TodoService::next_sort_key(&conn)?;
        
        conn.execute(
            "INSERT INTO todos (id, text, done, sort_key, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id.to_string(), text, 0, sort_key, now],
        )?;
        
        Ok(TodoItem {
            id,
            text: text.to_string(),
            done: false,
            sort_key,
            created_at: now,
        })
    }

    pub fn update(library: &Path, id: Uuid, text: Option<&str>, done: Option<bool>) -> Result<()> {
        let conn = Connection::open(library_db(library))?;
        
        if let Some(text) = text {
            conn.execute(
                "UPDATE todos SET text = ?1 WHERE id = ?2",
                params![text, id.to_string()],
            )?;
        }
        
        if let Some(done) = done {
            conn.execute(
                "UPDATE todos SET done = ?1 WHERE id = ?2",
                params![done as i32, id.to_string()],
            )?;
        }
        
        Ok(())
    }

    pub fn delete(library: &Path, id: Uuid) -> Result<()> {
        let conn = Connection::open(library_db(library))?;
        conn.execute("DELETE FROM todos WHERE id = ?1", params![id.to_string()])?;
        Ok(())
    }

    pub fn reorder(library: &Path, id: Uuid, new_sort_key: i64) -> Result<()> {
        let conn = Connection::open(library_db(library))?;
        conn.execute(
            "UPDATE todos SET sort_key = ?1 WHERE id = ?2",
            params![new_sort_key, id.to_string()],
        )?;
        Ok(())
    }

    pub fn toggle_done(library: &Path, id: Uuid) -> Result<bool> {
        let conn = Connection::open(library_db(library))?;
        let current: i32 = conn.query_row(
            "SELECT done FROM todos WHERE id = ?1",
            [id.to_string()],
            |row| row.get(0),
        )?;
        let new_done = current != 1;
        conn.execute(
            "UPDATE todos SET done = ?1 WHERE id = ?2",
            params![new_done as i32, id.to_string()],
        )?;
        Ok(new_done)
    }

    fn next_sort_key(conn: &Connection) -> Result<i64> {
        let max: Option<i64> = conn.query_row(
            "SELECT MAX(sort_key) FROM todos",
            [],
            |row| row.get(0),
        )?;
        Ok(max.map(|m| m + 1).unwrap_or(0))
    }
}
