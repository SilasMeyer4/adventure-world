use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;
use std::io::Write;
use tauri::{command, State};
use std::sync::{Mutex, Arc};


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    db_path: String, 
    num: u32
}

impl Config {
    pub fn load_or_create_config(filepath: &str) -> Result<Self, String> {
        if Path::new(filepath).exists() {
            let data = fs::read_to_string(filepath).expect("Failed to read config");
            toml::de::from_str(&data).map_err(|e| e.to_string())
        }
        else {
            let default_config = Config {
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



}


// Tauri command to get config
#[tauri::command]
pub fn get_config() -> Config {
    Config::load_or_create_config("data/config.toml").expect("Failed to load or create config")
}

#[tauri::command]
pub fn get_config_as_string() -> String {
    fs::read_to_string("data/config.toml").expect("Failed to read config")
}

#[tauri::command]
pub fn set_config(config: State<'_, Arc<Mutex<Config>>>, db_path: String, num: u32) {
    let mut config = config.lock().expect("Failed to lock config");
    config.db_path = db_path;
    config.num = num;
}

// Tauri command to update config
#[tauri::command]
pub fn save_config(config: State<'_, Config>) {
    config.save("data/config.toml");
}