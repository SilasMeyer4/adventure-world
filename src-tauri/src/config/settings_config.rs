use serde::{Serialize, Deserialize};
use std::{collections::HashMap, fs};
use std::path::Path;
use std::io::Write;
use tauri::{command, State};
use std::sync::{Mutex, Arc};
use crate::config::ConfigFile;

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsConfig {
    pub db_path: String, 
    pub num: u32
}

impl ConfigFile for SettingsConfig {
    fn load_or_create(filepath: &str) -> Result<Self, String> {
        if Path::new(filepath).exists() {
            let data = fs::read_to_string(filepath).expect("Failed to read config");
            toml::de::from_str(&data).map_err(|e| e.to_string())
        }
        else {
            let default_config = SettingsConfig {
                db_path: "data/database.db".to_string(),
                num: 10
            };
            default_config.save(filepath);
            Ok(default_config)
        }
    }

    fn save(&self, filepath: &str) {
        let toml_data = toml::to_string_pretty(self).expect("Failed to serialize config");
        let mut file = fs::File::create(filepath).expect("Failed to create config file");
        file.write_all(toml_data.as_bytes()).expect("Failed to write config file");
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

}