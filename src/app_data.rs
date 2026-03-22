use std::path::PathBuf;

use etcetera::{AppStrategy, AppStrategyArgs, choose_app_strategy};
use keyring::Entry;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{constants::APP_NAME_ID, error::Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppData {
    version: u8,

    pub plex: PlexData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlexData {
    pub client_id: String,
}

pub enum SecureStore {
    PlexToken,
}

impl AppData {
    fn path() -> Result<PathBuf> {
        let strategy = choose_app_strategy(AppStrategyArgs {
            top_level_domain: "dev".to_string(),
            author: APP_NAME_ID.to_string(),
            app_name: APP_NAME_ID.to_string(),
        })?;

        let config_dir = strategy.config_dir();
        std::fs::create_dir_all(&config_dir)?;
        Ok(config_dir.join("app_data.json"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::path()?;

        if !path.exists() {
            let default = Self::default();
            default.save()?;
            return Ok(default);
        }

        let contents = std::fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&contents)?)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;
        let contents = serde_json::to_string(self)?;
        std::fs::write(&path, contents)?;
        Ok(())
    }

    pub fn update(&mut self, f: impl FnOnce(&mut Self)) -> Result<()> {
        f(self);
        self.save()
    }
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            version: 0,
            plex: PlexData::default(),
        }
    }
}

impl Default for PlexData {
    fn default() -> Self {
        Self {
            client_id: Uuid::new_v4().to_string(),
        }
    }
}

impl SecureStore {
    pub fn service(&self) -> &'static str {
        match self {
            SecureStore::PlexToken => "plex_token",
        }
    }

    pub fn get(&self) -> Result<String> {
        let entry = Entry::new(self.service(), APP_NAME_ID)?;
        Ok(entry.get_password()?)
    }

    pub fn set(&self, value: impl AsRef<str>) -> Result<()> {
        let entry = Entry::new(self.service(), APP_NAME_ID)?;
        entry.set_password(value.as_ref())?;
        Ok(())
    }

    pub fn delete(&self) -> Result<()> {
        let entry = Entry::new(self.service(), APP_NAME_ID)?;
        Ok(entry.delete_credential()?)
    }
}
