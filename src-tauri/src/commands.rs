use anode_core::{
    count_words_from_doc, default_library_path, get_config, is_first_run, AppConfig, BookMeta,
    BookSummary, CompileOrderEntry, PageBody, PageKind, PageMeta,
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
    let book_id = Uuid::parse_str(&book_id).map_err(|e| e.to_string())?;
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
