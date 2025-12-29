//! Database Layer
//! 
//! SQLite-based database with migration support.
//! Shared by Core and all Apps.

use rusqlite::{Connection, Result as SqlResult};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Database wrapper with thread-safe connection
pub struct Database {
    conn: Mutex<Connection>,
}

/// Person entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: i64,
    pub name: String,
    pub notes: Option<String>,
    pub is_active: bool,
    pub created_at: String,
}

/// Conversation entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: i64,
    pub person_id: i64,
    pub content: String,
    pub context: Option<String>,
    pub created_at: String,
}

/// Memory entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: i64,
    pub person_id: i64,
    pub key: String,
    pub value: String,
    pub importance: i32,
    pub created_at: String,
}

impl Database {
    /// Create a new database connection
    pub fn new() -> SqlResult<Self> {
        let conn = Connection::open("corelab.db")?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Run database migrations
    pub fn run_migrations(&self) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        
        // Create migrations table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS _migrations (
                id INTEGER PRIMARY KEY,
                version TEXT NOT NULL UNIQUE,
                applied_at TEXT NOT NULL
            )",
            [],
        )?;

        // Migration 001: Create core tables
        self.apply_migration(&conn, "001_initial", r#"
            CREATE TABLE IF NOT EXISTS persons (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                notes TEXT,
                is_active INTEGER NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS conversations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                person_id INTEGER NOT NULL,
                content TEXT NOT NULL,
                context TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                FOREIGN KEY (person_id) REFERENCES persons(id)
            );

            CREATE TABLE IF NOT EXISTS memories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                person_id INTEGER NOT NULL,
                key TEXT NOT NULL,
                value TEXT NOT NULL,
                importance INTEGER NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                FOREIGN KEY (person_id) REFERENCES persons(id)
            );

            CREATE INDEX IF NOT EXISTS idx_conversations_person ON conversations(person_id);
            CREATE INDEX IF NOT EXISTS idx_memories_person ON memories(person_id);
        "#)?;

        Ok(())
    }

    fn apply_migration(&self, conn: &Connection, version: &str, sql: &str) -> SqlResult<()> {
        // Check if already applied
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM _migrations WHERE version = ?",
            [version],
            |row| row.get(0),
        )?;

        if count == 0 {
            conn.execute_batch(sql)?;
            conn.execute(
                "INSERT INTO _migrations (version, applied_at) VALUES (?, datetime('now'))",
                [version],
            )?;
            println!("Applied migration: {}", version);
        }

        Ok(())
    }

    // Person CRUD
    pub fn get_persons(&self) -> SqlResult<Vec<Person>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, notes, is_active, created_at FROM persons WHERE is_active = 1")?;
        let persons = stmt.query_map([], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                notes: row.get(2)?,
                is_active: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;
        persons.collect()
    }

    pub fn create_person(&self, name: &str, notes: Option<&str>) -> SqlResult<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO persons (name, notes) VALUES (?, ?)",
            rusqlite::params![name, notes],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn update_person(&self, id: i64, name: &str, notes: Option<&str>, is_active: bool) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE persons SET name = ?, notes = ?, is_active = ? WHERE id = ?",
            rusqlite::params![name, notes, is_active, id],
        )?;
        Ok(())
    }

    // Conversation CRUD
    pub fn get_conversations(&self, person_id: i64) -> SqlResult<Vec<Conversation>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, person_id, content, context, created_at FROM conversations WHERE person_id = ? ORDER BY created_at DESC"
        )?;
        let convs = stmt.query_map([person_id], |row| {
            Ok(Conversation {
                id: row.get(0)?,
                person_id: row.get(1)?,
                content: row.get(2)?,
                context: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;
        convs.collect()
    }

    pub fn create_conversation(&self, person_id: i64, content: &str, context: Option<&str>) -> SqlResult<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO conversations (person_id, content, context) VALUES (?, ?, ?)",
            rusqlite::params![person_id, content, context],
        )?;
        Ok(conn.last_insert_rowid())
    }

    // Memory CRUD
    pub fn get_memories(&self, person_id: i64) -> SqlResult<Vec<Memory>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, person_id, key, value, importance, created_at FROM memories WHERE person_id = ? ORDER BY importance DESC"
        )?;
        let mems = stmt.query_map([person_id], |row| {
            Ok(Memory {
                id: row.get(0)?,
                person_id: row.get(1)?,
                key: row.get(2)?,
                value: row.get(3)?,
                importance: row.get(4)?,
                created_at: row.get(5)?,
            })
        })?;
        mems.collect()
    }

    pub fn create_memory(&self, person_id: i64, key: &str, value: &str, importance: i32) -> SqlResult<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO memories (person_id, key, value, importance) VALUES (?, ?, ?, ?)",
            rusqlite::params![person_id, key, value, importance],
        )?;
        Ok(conn.last_insert_rowid())
    }
}
