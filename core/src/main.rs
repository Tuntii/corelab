#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use corelab_lib::modules::database::Database;

fn main() {
    // Initialize database
    let db = Database::new().expect("Failed to initialize database");
    db.run_migrations().expect("Failed to run migrations");

    tauri::Builder::default()
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            corelab_lib::commands::get_persons,
            corelab_lib::commands::create_person,
            corelab_lib::commands::update_person,
            corelab_lib::commands::get_conversations,
            corelab_lib::commands::create_conversation,
            corelab_lib::commands::get_memories,
            corelab_lib::commands::create_memory,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
