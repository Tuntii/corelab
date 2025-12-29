//! CoreLab - Core Layer Library
//! 
//! This crate provides the foundation for CoreLab applications:
//! - Database layer (SQLite)
//! - App Registry
//! - Event System
//! - AI Interface

pub mod modules;
pub mod commands;

pub use modules::database::Database;
pub use modules::registry::AppRegistry;
pub use modules::events::EventBus;
pub use modules::ai::AIProvider;
