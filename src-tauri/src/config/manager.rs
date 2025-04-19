use crate::config_init;

use super::save_data_config::SaveDataConfig;
use super::settings_config::SettingsConfig;
use super::{
    config::{ConfigFile, ConfigType},
    save_data_config,
};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tauri::{command, State};

pub struct ConfigManager {
    configs: HashMap<ConfigType, Box<dyn ConfigFile>>,
    settings_path: String,
    save_data_path: String,
}

impl ConfigManager {
    pub fn new(resource_path: &str) -> Self {
        let mut configs: HashMap<ConfigType, Box<dyn ConfigFile>> = HashMap::new();

        // Construct paths once using PathBuf and keep them that way
        let settings_path = Path::new(resource_path).join("data/settings.toml");
        let save_data_path = Path::new(resource_path).join("data/save_data.toml");

        // Load configs using Path -> &str (only when needed)
        let settings = SettingsConfig::load_or_create(
            settings_path.to_str().expect("Invalid UTF-8 in settings path")
        ).expect("Failed to load settings");

        let save_data = SaveDataConfig::load_or_create(
            save_data_path.to_str().expect("Invalid UTF-8 in save data path")
        ).expect("Failed to load save data");

        configs.insert(ConfigType::Settings, Box::new(settings));
        configs.insert(ConfigType::SaveData, Box::new(save_data));

        Self {
            configs,
            settings_path: settings_path.to_string_lossy().into_owned(),
            save_data_path: save_data_path.to_string_lossy().into_owned(),
        }
    }

    pub fn get<T: 'static>(&self, key: &ConfigType) -> Option<&T> {
        self.configs.get(key)?.as_any().downcast_ref::<T>()
    }

    pub fn get_mut<T: 'static>(&mut self, key: &ConfigType) -> Option<&mut T> {
        self.configs.get_mut(key)?.as_any_mut().downcast_mut::<T>()
    }

    pub fn save_all(&self) {
        if let Some(settings_config) = self.configs.get(&ConfigType::Settings) {
            settings_config.save(&self.settings_path); // Save the Settings config
        }

        if let Some(save_data_config) = self.configs.get(&ConfigType::SaveData) {
            save_data_config.save(&self.save_data_path); // Save the Settings config
        }
    }
}

#[tauri::command]
pub fn set_settings_config(
    config_manager: State<'_, Arc<Mutex<ConfigManager>>>,
    db_path: String,
    num: u32,
) {
    let mut config_manager = config_manager
        .lock()
        .map_err(|e| e.to_string())
        .expect("Failed to lock config_manager");

    let boxed_config: &mut Box<dyn ConfigFile> = config_manager
        .get_mut(&ConfigType::Settings)
        .expect("Failed to get settings config");

    let config: &mut SettingsConfig = boxed_config
        .as_any_mut()
        .downcast_mut::<SettingsConfig>()
        .expect("Failed to downcast to SettingsConfig");

    config.db_path = db_path;
    config.num = num;
}

#[tauri::command]
pub fn set_save_data_config(
    config_manager: State<'_, Arc<Mutex<ConfigManager>>>,
    test_val: String,
) {
    let mut config_manager = config_manager
        .lock()
        .map_err(|e| e.to_string())
        .expect("Failed to lock config_manager");

    let boxed_config: &mut Box<dyn ConfigFile> = config_manager
        .get_mut(&ConfigType::SaveData)
        .expect("Failed to get settings config");

    let config: &mut SaveDataConfig = boxed_config
        .as_any_mut()
        .downcast_mut::<SaveDataConfig>()
        .expect("Failed to downcast to SettingsConfig");

    config.test_val = test_val;
}

#[tauri::command]
pub fn save_configs(config_manager: State<'_, Arc<Mutex<ConfigManager>>>) {
    let config_manager = config_manager
        .lock()
        .map_err(|e| e.to_string())
        .expect("Failed to lock config_manager");

    config_manager.save_all();
}
