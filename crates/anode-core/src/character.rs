use std::path::Path;
use chrono::Utc;
use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::models::Character;
use crate::Result;

pub struct CharacterService;

impl CharacterService {
    pub fn list(library: &Path, book_id: Uuid) -> Result<Vec<Character>> {
        let conn = crate::book::BookService::open_db(library, book_id)?;
        let mut stmt = conn.prepare(
            "SELECT id, name, role, description, notes, updated_at FROM characters ORDER BY name ASC"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(Character {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap_or_else(|_| Uuid::new_v4()),
                name: row.get(1)?,
                role: row.get(2)?,
                description: row.get(3)?,
                notes: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;

        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn create(library: &Path, book_id: Uuid, name: &str) -> Result<Character> {
        let conn = crate::book::BookService::open_db(library, book_id)?;
        let id = Uuid::new_v4();
        let now = Utc::now().to_rfc3339();
        let char = Character {
            id,
            name: name.to_string(),
            role: String::new(),
            description: String::new(),
            notes: String::new(),
            updated_at: now.clone(),
        };

        conn.execute(
            "INSERT INTO characters (id, name, role, description, notes, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                id.to_string(),
                char.name,
                char.role,
                char.description,
                char.notes,
                now,
            ],
        )?;

        Ok(char)
    }

    pub fn update(
        library: &Path,
        book_id: Uuid,
        id: Uuid,
        name: Option<String>,
        role: Option<String>,
        description: Option<String>,
        notes: Option<String>,
    ) -> Result<()> {
        let conn = crate::book::BookService::open_db(library, book_id)?;
        let mut updates = Vec::new();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(n) = name {
            updates.push("name = ?");
            params.push(Box::new(n));
        }
        if let Some(r) = role {
            updates.push("role = ?");
            params.push(Box::new(r));
        }
        if let Some(d) = description {
            updates.push("description = ?");
            params.push(Box::new(d));
        }
        if let Some(nt) = notes {
            updates.push("notes = ?");
            params.push(Box::new(nt));
        }

        if updates.is_empty() {
            return Ok(());
        }

        updates.push("updated_at = ?");
        params.push(Box::new(Utc::now().to_rfc3339()));

        let sql = format!(
            "UPDATE characters SET {} WHERE id = ?",
            updates.join(", ")
        );
        params.push(Box::new(id.to_string()));

        let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        conn.execute(&sql, rusqlite::params_from_iter(param_refs))?;

        Ok(())
    }

    pub fn delete(library: &Path, book_id: Uuid, id: Uuid) -> Result<()> {
        let conn = crate::book::BookService::open_db(library, book_id)?;
        conn.execute("DELETE FROM characters WHERE id = ?1", params![id.to_string()])?;
        Ok(())
    }
}
