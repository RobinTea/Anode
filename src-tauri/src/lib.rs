mod commands;
mod state;

use state::{AppState, SharedState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::new().expect("failed to init app state");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(SharedState::new(app_state))
        .invoke_handler(tauri::generate_handler![
            commands::is_first_run_cmd,
            commands::get_config_cmd,
            commands::init_library_cmd,
            commands::get_default_library_path,
            commands::list_books,
            commands::create_book,
            commands::delete_book,
            commands::get_book_meta,
            commands::list_pages,
            commands::create_page,
            commands::load_page_body,
            commands::save_page_body,
            commands::get_compile_order,
            commands::set_compile_order,
            commands::update_page_meta,
            commands::list_snapshots,
            commands::restore_snapshot,
            commands::export_book_cmd,
            commands::import_book_cmd,
            commands::export_docx_cmd,
            commands::list_todos,
            commands::create_todo,
            commands::update_todo,
            commands::delete_todo,
            commands::toggle_todo_done,
            commands::get_daily_quest,
            commands::get_weekly_quests,
            commands::get_history_quests,
            commands::list_characters,
            commands::create_character,
            commands::update_character,
            commands::delete_character,
            commands::search_pages,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
