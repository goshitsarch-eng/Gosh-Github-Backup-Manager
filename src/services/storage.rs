use crate::types::{AppSettings, BackupHistoryEntry};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct StorageData {
    token: Option<String>,
    settings: Option<AppSettings>,
    backup_history: Vec<BackupHistoryEntry>,
}

#[derive(Clone)]
pub struct StorageService {
    storage_path: PathBuf,
}

impl StorageService {
    pub fn new() -> Self {
        let base_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("gosh-github-backup-manager");

        let storage_path = base_dir.join("gosh-github-backup-manager.json");

        if let Some(parent) = storage_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        Self { storage_path }
    }

    fn load_data(&self) -> StorageData {
        if self.storage_path.exists() {
            match fs::read_to_string(&self.storage_path) {
                Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
                Err(e) => {
                    log::warn!("Failed to read storage file: {}", e);
                    StorageData::default()
                }
            }
        } else {
            StorageData::default()
        }
    }

    fn save_data(&self, data: &StorageData) -> Result<(), String> {
        let json = serde_json::to_string_pretty(data)
            .map_err(|e| format!("Failed to serialize data: {}", e))?;

        fs::write(&self.storage_path, &json)
            .map_err(|e| format!("Failed to write storage file: {}", e))?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(&self.storage_path, fs::Permissions::from_mode(0o600));
        }

        log::info!("Saved data to {:?}", self.storage_path);
        Ok(())
    }

    pub fn get_token(&self) -> Result<Option<String>, String> {
        let data = self.load_data();
        Ok(data.token)
    }

    pub fn set_token(&self, token: &str) -> Result<(), String> {
        log::info!("set_token: saving token ({} chars)", token.len());
        let mut data = self.load_data();
        data.token = Some(token.to_string());
        self.save_data(&data)
    }

    pub fn clear_token(&self) -> Result<(), String> {
        log::info!("clear_token: clearing token");
        let mut data = self.load_data();
        data.token = None;
        self.save_data(&data)
    }

    pub fn get_settings(&self) -> Result<AppSettings, String> {
        Ok(self.load_data().settings.unwrap_or_default())
    }

    pub fn set_settings(&self, settings: &AppSettings) -> Result<(), String> {
        let mut data = self.load_data();
        data.settings = Some(settings.clone());
        self.save_data(&data)
    }

    pub fn get_backup_history(&self) -> Result<Vec<BackupHistoryEntry>, String> {
        Ok(self.load_data().backup_history)
    }

    pub fn add_backup_history(&self, entry: BackupHistoryEntry) -> Result<(), String> {
        let mut data = self.load_data();
        data.backup_history.insert(0, entry);
        data.backup_history.truncate(50);
        self.save_data(&data)
    }

    pub fn clear_backup_history(&self) -> Result<(), String> {
        let mut data = self.load_data();
        data.backup_history.clear();
        self.save_data(&data)
    }
}
