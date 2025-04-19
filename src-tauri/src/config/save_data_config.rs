use crate::config::ConfigFile;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, fs};
use tauri::{command, State};

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveDataConfig {
    pub test_val: String,
}

impl ConfigFile for SaveDataConfig {
    fn load_or_create(filepath: &str) -> Result<Self, String> {
        if Path::new(filepath).exists() {
            let data = fs::read_to_string(filepath).expect("Failed to read config");
            toml::de::from_str(&data).map_err(|e| e.to_string())
        } else {
            let default_config = SaveDataConfig {
                test_val: "BibubTest".to_string(),
            };
            default_config.save(filepath);
            Ok(default_config)
        }
    }

    fn save(&self, filepath: &str) {
        let toml_data = toml::to_string_pretty(self).expect("Failed to serialize config");
        let mut file = fs::File::create(filepath).expect("Failed to create config file");
        file.write_all(toml_data.as_bytes())
            .expect("Failed to write config file");
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
