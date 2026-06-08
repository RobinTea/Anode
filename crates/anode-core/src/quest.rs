use std::path::Path;

use chrono::Utc;
use rusqlite::{params, Connection};

use crate::models::DailyQuest;
use crate::paths::library_db;
use crate::{AnodeError, Result};

pub struct QuestService;

impl QuestService {
    const DEFAULT_GOAL: u64 = 500; // words per day

    pub fn get_today(library: &Path) -> Result<DailyQuest> {
        let conn = Connection::open(library_db(library))?;
        let today = Utc::now().date_naive().to_string();
        
        let row = conn.query_row(
            "SELECT date, word_count FROM daily_words WHERE date = ?1",
            [today.clone()],
            |row| {
                Ok(DailyQuest {
                    date: row.get(0)?,
                    word_count: row.get(1)?,
                    goal: Self::DEFAULT_GOAL,
                })
            }
        );
        
        match row {
            Ok(q) => Ok(q),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                // Create today's quest
                let quest = DailyQuest {
                    date: today,
                    word_count: 0,
                    goal: Self::DEFAULT_GOAL,
                };
                Self::save(library, &quest)?;
                Ok(quest)
            }
            Err(e) => Err(AnodeError::msg(e.to_string())),
        }
    }

    pub fn save(library: &Path, quest: &DailyQuest) -> Result<()> {
        let conn = Connection::open(library_db(library))?;
        conn.execute(
            "INSERT OR REPLACE INTO daily_words (date, word_count) VALUES (?1, ?2)",
            params![quest.date.clone(), quest.word_count],
        )?;
        Ok(())
    }

    pub fn add_words(library: &Path, count: u64) -> Result<DailyQuest> {
        let mut quest = Self::get_today(library)?;
        quest.word_count += count;
        Self::save(library, &quest)?;
        Ok(quest)
    }

    pub fn set_goal(_library: &Path, _goal: u64) -> Result<()> {
        // Store goal in config or separate table
        // For now, we'll use a simple approach
        Ok(())
    }

    pub fn get_weekly(library: &Path) -> Result<Vec<DailyQuest>> {
        Self::get_history(library, 7)
    }

    pub fn get_history(library: &Path, days: u32) -> Result<Vec<DailyQuest>> {
        let conn = Connection::open(library_db(library))?;
        let today = Utc::now().date_naive();
        let start_date = today - chrono::Duration::days((days - 1) as i64);
        
        let mut stmt = conn.prepare(
            "SELECT date, word_count FROM daily_words WHERE date >= ?1 ORDER BY date ASC"
        )?;
        
        let mut quests = Vec::new();
        let mut rows = stmt.query([start_date.to_string()])?;
        while let Some(row) = rows.next()? {
            quests.push(DailyQuest {
                date: row.get(0)?,
                word_count: row.get(1)?,
                goal: Self::DEFAULT_GOAL,
            });
        }
        
        Ok(quests)
    }
}
