use anode_core::{
    count_words_from_doc, default_library_path, get_config, is_first_run, AppConfig, BookMeta,
    BookSummary, CompileOrderEntry, DailyQuest, PageBody, PageKind, PageMeta, TodoItem,
};
use tauri::State;
use uuid::Uuid;

use crate::state::SharedState;

#[tauri::command]
pub fn is_first_run_cmd() -> Result<bool, String> {
    is_first_run().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_config_cmd() -> Result<AppConfig, String> {
    get_config().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn init_library_cmd(path: String, state: State<'_, SharedState>) -> Result<(), String> {
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .init_library(path.into())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_default_library_path() -> String {
    default_library_path().to_string_lossy().into_owned()
}

#[tauri::command]
pub fn list_books(state: State<'_, SharedState>) -> Result<Vec<BookSummary>, String> {
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard.ensure_library().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| lib.list_books())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_book(title: String, state: State<'_, SharedState>) -> Result<BookMeta, String> {
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard.ensure_library().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| lib.create_book(&title))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_book(book_id: String, state: State<'_, SharedState>) -> Result<(), String> {
    let id = Uuid::parse_str(&book_id).map_err(|e| e.to_string())?;
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| lib.delete_book(id))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_book_meta(book_id: String, state: State<'_, SharedState>) -> Result<BookMeta, String> {
    let id = Uuid::parse_str(&book_id).map_err(|e| e.to_string())?;
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| anode_core::BookService::load_meta(lib.path(), id))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_pages(book_id: String, state: State<'_, SharedState>) -> Result<Vec<PageMeta>, String> {
    let id = Uuid::parse_str(&book_id).map_err(|e| e.to_string())?;
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| anode_core::PageService::list(lib.path(), id))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_page(
    book_id: String,
    kind: String,
    class: String,
    title: String,
    state: State<'_, SharedState>,
) -> Result<PageMeta, String> {
    let id = Uuid::parse_str(&book_id).map_err(|e| e.to_string())?;
    let kind = match kind.as_str() {
        "plan" => PageKind::Plan,
        "read" => PageKind::Read,
        _ => PageKind::Write,
    };
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| anode_core::PageService::create(lib.path(), id, kind, &class, &title))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_page_body(
    book_id: String,
    page_id: String,
    state: State<'_, SharedState>,
) -> Result<PageBody, String> {
    let book_id = Uuid::parse_str(&book_id).map_err(|e| e.to_string())?;
    let page_id = Uuid::parse_str(&page_id).map_err(|e| e.to_string())?;
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| anode_core::PageService::load_body(lib.path(), book_id, page_id))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_page_body(
    book_id: String,
    page_id: String,
    doc: serde_json::Value,
    plain_text: String,
    state: State<'_, SharedState>,
) -> Result<PageBody, String> {
    let book_id = Uuid::parse_str(&book_id).map_err(|e| e.to_string())?;
    let page_id = Uuid::parse_str(&page_id).map_err(|e| e.to_string())?;
    let word_count = count_words_from_doc(&doc);
    let body = PageBody {
        format: "tiptap".into(),
        format_version: 1,
        doc,
        plain_text_cache: plain_text,
        word_count,
    };
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| {
            anode_core::PageService::save_body(lib.path(), book_id, page_id, &body)?;
            lib.touch_book(book_id)?;
            Ok(body)
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_compile_order(
    book_id: String,
    state: State<'_, SharedState>,
) -> Result<Vec<CompileOrderEntry>, String> {
    let id = Uuid::parse_str(&book_id).map_err(|e| e.to_string())?;
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| anode_core::PageService::compile_order(lib.path(), id))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_compile_order(
    book_id: String,
    entries: Vec<CompileOrderEntry>,
    state: State<'_, SharedState>,
) -> Result<(), String> {
    let id = Uuid::parse_str(&book_id).map_err(|e| e.to_string())?;
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| anode_core::PageService::update_compile_order(lib.path(), id, &entries))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_page_meta(
    book_id: String,
    page_id: String,
    meta: PartialPageMeta,
    state: State<'_, SharedState>,
) -> Result<(), String> {
    let book_id = Uuid::parse_str(&book_id).map_err(|e| e.to_string())?;
    let page_id = Uuid::parse_str(&page_id).map_err(|e| e.to_string())?;
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| {
            anode_core::PageService::update_meta(
                lib.path(),
                book_id,
                page_id,
                meta.title,
                meta.sort_key,
                meta.status,
                meta.notes,
            )
        })
        .map_err(|e| e.to_string())
}

#[derive(serde::Deserialize)]
pub struct PartialPageMeta {
    pub title: Option<String>,
    pub sort_key: Option<i64>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

#[tauri::command]
pub fn list_snapshots(
    book_id: String,
    page_id: String,
    state: State<'_, SharedState>,
) -> Result<Vec<anode_core::SnapshotInfo>, String> {
    let book_id = Uuid::parse_str(&book_id).map_err(|e| e.to_string())?;
    let page_id = Uuid::parse_str(&page_id).map_err(|e| e.to_string())?;
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| {
            let book_dir = anode_core::paths::book_dir(lib.path(), book_id);
            anode_core::PageService::list_snapshots(&book_dir, page_id)
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn restore_snapshot(
    book_id: String,
    page_id: String,
    filename: String,
    state: State<'_, SharedState>,
) -> Result<PageBody, String> {
    let book_id = Uuid::parse_str(&book_id).map_err(|e| e.to_string())?;
    let page_id = Uuid::parse_str(&page_id).map_err(|e| e.to_string())?;
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| {
            anode_core::PageService::restore_snapshot(lib.path(), book_id, page_id, &filename)?;
            anode_core::PageService::load_body(lib.path(), book_id, page_id)
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn export_book_cmd(
    book_id: String,
    include_snapshots: bool,
    output_path: String,
    state: State<'_, SharedState>,
) -> Result<(), String> {
    let book_id = if book_id == "library-backup" {
        Uuid::nil()
    } else {
        Uuid::parse_str(&book_id).map_err(|e| e.to_string())?
    };
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| {
            anode_core::export_book(lib.path(), book_id, include_snapshots, output_path.as_ref())
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn import_book_cmd(
    anode_path: String,
    state: State<'_, SharedState>,
) -> Result<String, String> {
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    let book_id = guard
        .with_library(|lib| anode_core::import_book(lib.path(), anode_path.as_ref()))
        .map_err(|e| e.to_string())?;
    Ok(book_id.to_string())
}

#[tauri::command]
pub fn export_docx_cmd(
    book_id: String,
    output_path: String,
    state: State<'_, SharedState>,
) -> Result<(), String> {
    let book_id = Uuid::parse_str(&book_id).map_err(|e| e.to_string())?;
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| {
            anode_core::compile_to_docx(lib.path(), book_id, false, output_path.as_ref())
        })
        .map_err(|e| e.to_string())
}

// Todo commands
#[tauri::command]
pub fn list_todos(state: State<'_, SharedState>) -> Result<Vec<TodoItem>, String> {
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| anode_core::TodoService::list(lib.path()))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_todo(text: String, state: State<'_, SharedState>) -> Result<TodoItem, String> {
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| anode_core::TodoService::create(lib.path(), &text))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_todo(
    id: String,
    text: Option<String>,
    done: Option<bool>,
    state: State<'_, SharedState>,
) -> Result<(), String> {
    let id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| {
            anode_core::TodoService::update(lib.path(), id, text.as_deref(), done)
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_todo(id: String, state: State<'_, SharedState>) -> Result<(), String> {
    let id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| anode_core::TodoService::delete(lib.path(), id))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_todo_done(id: String, state: State<'_, SharedState>) -> Result<bool, String> {
    let id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| anode_core::TodoService::toggle_done(lib.path(), id))
        .map_err(|e| e.to_string())
}

// Quest commands
#[tauri::command]
pub fn get_daily_quest(state: State<'_, SharedState>) -> Result<DailyQuest, String> {
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| anode_core::QuestService::get_today(lib.path()))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_weekly_quests(state: State<'_, SharedState>) -> Result<Vec<DailyQuest>, String> {
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| anode_core::QuestService::get_weekly(lib.path()))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_history_quests(days: u32, state: State<'_, SharedState>) -> Result<Vec<DailyQuest>, String> {
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| anode_core::QuestService::get_history(lib.path(), days))
        .map_err(|e| e.to_string())
}

// Character commands
#[tauri::command]
pub fn list_characters(book_id: String, state: State<'_, SharedState>) -> Result<Vec<anode_core::Character>, String> {
    let book_id = Uuid::parse_str(&book_id).map_err(|e| e.to_string())?;
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| anode_core::CharacterService::list(lib.path(), book_id))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_character(book_id: String, name: String, state: State<'_, SharedState>) -> Result<anode_core::Character, String> {
    let book_id = Uuid::parse_str(&book_id).map_err(|e| e.to_string())?;
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| anode_core::CharacterService::create(lib.path(), book_id, &name))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_character(
    book_id: String,
    id: String,
    name: Option<String>,
    role: Option<String>,
    description: Option<String>,
    notes: Option<String>,
    state: State<'_, SharedState>,
) -> Result<(), String> {
    let book_id = Uuid::parse_str(&book_id).map_err(|e| e.to_string())?;
    let id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| {
            anode_core::CharacterService::update(lib.path(), book_id, id, name, role, description, notes)
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_character(book_id: String, id: String, state: State<'_, SharedState>) -> Result<(), String> {
    let book_id = Uuid::parse_str(&book_id).map_err(|e| e.to_string())?;
    let id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| anode_core::CharacterService::delete(lib.path(), book_id, id))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn search_pages(book_id: String, query: String, state: State<'_, SharedState>) -> Result<Vec<PageMeta>, String> {
    let book_id = Uuid::parse_str(&book_id).map_err(|e| e.to_string())?;
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    guard
        .with_library(|lib| {
            let conn = anode_core::BookService::open_db(lib.path(), book_id)?;
            let mut stmt = conn.prepare(
                "SELECT id, kind, class, title, sort_key, status, word_count, notes, updated_at FROM pages 
                 WHERE title LIKE ?1 OR content LIKE ?1 ORDER BY sort_key ASC"
            )?;
            let pattern = format!("%{}%", query);
            let rows = stmt.query_map([pattern], |row| {
                Ok(PageMeta {
                    id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    kind: anode_core::str_to_kind(&row.get::<_, String>(1)?),
                    class: row.get(2)?,
                    title: row.get(3)?,
                    sort_key: row.get(4)?,
                    status: row.get(5)?,
                    word_count: row.get(6)?,
                    notes: row.get(7)?,
                    updated_at: row.get::<_, String>(8)?.parse().unwrap_or_else(|_| chrono::Utc::now()),
                })
            })?;

            let mut out = Vec::new();
            for row in rows {
                out.push(row?);
            }
            Ok(out)
        })
        .map_err(|e| e.to_string())
}
