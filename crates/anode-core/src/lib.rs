//! Anode core — local-first book library, pages, and persistence.

mod book;
mod error;
mod library;
mod models;
mod page;
mod export;
mod compile;
mod todo;
mod quest;
mod character;
pub mod paths;
mod schema;

pub use book::BookService;
pub use error::{AnodeError, Result};
pub use library::{init_library_at, is_first_run, get_config, set_library_path, write_lock_marker, Library};
pub use models::*;
pub use page::{count_words_from_doc, PageService, kind_to_str, str_to_kind};
pub use paths::default_library_path;
pub use export::{export_book, import_book};
pub use compile::compile_to_docx;
pub use todo::TodoService;
pub use quest::QuestService;
pub use character::CharacterService;
