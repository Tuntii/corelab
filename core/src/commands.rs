//! Tauri IPC Commands
//! 
//! These functions are exposed to the TypeScript frontend.

use crate::modules::database::{Database, Person, Conversation, Memory};
use tauri::State;

/// Get all active persons
#[tauri::command]
pub fn get_persons(db: State<Database>) -> Result<Vec<Person>, String> {
    db.get_persons().map_err(|e| e.to_string())
}

/// Create a new person
#[tauri::command]
pub fn create_person(
    db: State<Database>,
    name: String,
    notes: Option<String>,
) -> Result<i64, String> {
    db.create_person(&name, notes.as_deref())
        .map_err(|e| e.to_string())
}

/// Update a person
#[tauri::command]
pub fn update_person(
    db: State<Database>,
    id: i64,
    name: String,
    notes: Option<String>,
    is_active: bool,
) -> Result<(), String> {
    db.update_person(id, &name, notes.as_deref(), is_active)
        .map_err(|e| e.to_string())
}

/// Get conversations for a person
#[tauri::command]
pub fn get_conversations(
    db: State<Database>,
    person_id: i64,
) -> Result<Vec<Conversation>, String> {
    db.get_conversations(person_id).map_err(|e| e.to_string())
}

/// Create a new conversation
#[tauri::command]
pub fn create_conversation(
    db: State<Database>,
    person_id: i64,
    content: String,
    context: Option<String>,
) -> Result<i64, String> {
    db.create_conversation(person_id, &content, context.as_deref())
        .map_err(|e| e.to_string())
}

/// Get memories for a person
#[tauri::command]
pub fn get_memories(db: State<Database>, person_id: i64) -> Result<Vec<Memory>, String> {
    db.get_memories(person_id).map_err(|e| e.to_string())
}

/// Create a new memory
#[tauri::command]
pub fn create_memory(
    db: State<Database>,
    person_id: i64,
    key: String,
    value: String,
    importance: i32,
) -> Result<i64, String> {
    db.create_memory(person_id, &key, &value, importance)
        .map_err(|e| e.to_string())
}
