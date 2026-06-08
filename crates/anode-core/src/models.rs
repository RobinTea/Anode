use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PageKind {
    Plan,
    Write,
    Read,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub library_path: Option<String>,
    pub first_run_complete: bool,
    #[serde(default)]
    pub theme: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            library_path: None,
            first_run_complete: false,
            theme: "system".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookMeta {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub genre: String,
    pub synopsis: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookSummary {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub updated_at: DateTime<Utc>,
    pub write_page_count: u32,
    pub total_words: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageMeta {
    pub id: Uuid,
    pub kind: PageKind,
    pub class: String,
    pub title: String,
    pub sort_key: i64,
    pub status: String,
    pub word_count: u64,
    #[serde(default)]
    pub notes: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageBody {
    pub format: String,
    pub format_version: u32,
    pub doc: serde_json::Value,
    #[serde(default)]
    pub plain_text_cache: String,
    #[serde(default)]
    pub word_count: u64,
}

impl Default for PageBody {
    fn default() -> Self {
        Self {
            format: "tiptap".into(),
            format_version: 1,
            doc: serde_json::json!({ "type": "doc", "content": [] }),
            plain_text_cache: String::new(),
            word_count: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileOrderEntry {
    pub page_id: Uuid,
    pub position: i64,
    pub included: bool,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotInfo {
    pub filename: String,
    pub timestamp: String,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: Uuid,
    pub text: String,
    pub done: bool,
    pub sort_key: i64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyQuest {
    pub date: String,
    pub word_count: u64,
    pub goal: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: Uuid,
    pub name: String,
    pub role: String,
    pub description: String,
    pub notes: String,
    pub updated_at: String,
}
