use thiserror::Error;

pub type Result<T> = std::result::Result<T, AnodeError>;

#[derive(Debug, Error)]
pub enum AnodeError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Database error: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("{0}")]
    Message(String),
}

impl AnodeError {
    pub fn msg(s: impl Into<String>) -> Self {
        Self::Message(s.into())
    }
}
