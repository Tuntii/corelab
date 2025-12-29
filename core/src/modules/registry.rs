//! App Registry
//! 
//! Manages registered applications in CoreLab.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;

/// App registration info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
}

/// App Registry - manages all registered apps
pub struct AppRegistry {
    apps: RwLock<HashMap<String, AppInfo>>,
}

impl AppRegistry {
    pub fn new() -> Self {
        Self {
            apps: RwLock::new(HashMap::new()),
        }
    }

    /// Register a new app
    pub fn register(&self, app: AppInfo) -> Result<(), String> {
        let mut apps = self.apps.write().unwrap();
        if apps.contains_key(&app.id) {
            return Err(format!("App '{}' already registered", app.id));
        }
        apps.insert(app.id.clone(), app);
        Ok(())
    }

    /// Get registered app by ID
    pub fn get(&self, id: &str) -> Option<AppInfo> {
        let apps = self.apps.read().unwrap();
        apps.get(id).cloned()
    }

    /// List all registered apps
    pub fn list(&self) -> Vec<AppInfo> {
        let apps = self.apps.read().unwrap();
        apps.values().cloned().collect()
    }

    /// Unregister an app
    pub fn unregister(&self, id: &str) -> Option<AppInfo> {
        let mut apps = self.apps.write().unwrap();
        apps.remove(id)
    }
}

impl Default for AppRegistry {
    fn default() -> Self {
        Self::new()
    }
}
