// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::sync::{Mutex, Arc};

use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use tauri_plugin_updater::UpdaterExt;
use tauri::{AppHandle, Emitter, Manager};
use tauri::App;

mod database;
mod program;
mod database_manager;
mod config;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}


#[tauri::command]
fn greet(name: &str) -> String {
    format!(
        "Hello, {}! This is the last auto update test hopefully",
        name
    )
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let handle = app.handle().clone();
            database_init(app);
            config_init(app);
            save_init(app);
            updater_init(handle);
            
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet, 
            database::create_database,
            database::add_entity,
            database::add_character,
            database::add_item,
            database::add_place,
            database::add_encounter,
            database::add_encounter_entity,
            database::add_entity_item,
            config::get_config,
            config::save_config,
            config::get_config_as_string,
            config::set_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        println!("Found update, downloading...");

        // Show a custom dialog with "Absolutely" and "Totally" buttons
        let response = app.dialog()
            .message("A new version of the app is available. Would you like to update now?")
            .title("Update Available")
            .buttons(MessageDialogButtons::OkCancelCustom("Yes".to_string(), "No".to_string()))
            .blocking_show();

        if response {
            // User clicked "Absolutely" (button 1)
            // Proceed to download and install
            update.download_and_install(
                |chunk, total| println!("Downloaded {} of {:?}", chunk, total),
                || println!("Download complete"),
            ).await?;

            println!("Update installed!");
            app.restart();  // Restart the app after installation
        } else {
            // User clicked "Totally" (button 2) or canceled the dialog
            println!("Skipping the update.");
        }
    } else {
        println!("No Updates found.");
    }
    Ok(())
}


fn database_init(app: &App) {
    match database::Database::load_or_create_database("database.dnd") {
        Ok(db) => {
            app.manage(db);
            app.emit("database_load", "Loaded Database Sucessfully").unwrap(); //Does nothing?
            println!("Database loaded successfully");
        }
    
        Err(error) => {
            let error_msg = format!("Database failed to load: {}", error);
            println!("Error loading database: {}", error_msg);
            app.emit("database_load", format!("Failed to load Database {}", error_msg)).unwrap();
            
        }
    }
}

fn config_init(app: &App) {
    match config::Config::load_or_create_config("data/config.toml") {
        Ok(config) => {
            let config = Arc::new(Mutex::new(config));
            app.manage(config);
      
            app.emit("config_load", "Loaded Config Sucessfully").unwrap();
            println!("Config loaded successfully");
        }
    
        Err(error) => {
            let error_msg = format!("Config failed to load: {}", error);
            println!("Error loading Config: {}", error_msg);
            app.emit("config_load", format!("Failed to load Config {}", error_msg)).unwrap();
            
        }
    }
}

fn save_init(app: &App) {
    match config::Config::load_or_create_config("data/save.toml") {
        Ok(save) => {
            app.manage(save);
      
            app.emit("save_load", "Loaded Save Sucessfully").unwrap();
            println!("config loaded successfully");
        }
    
        Err(error) => {
            let error_msg = format!("Save failed to load: {}", error);
            println!("Error loading Save: {}", error_msg);
            app.emit("save_load", format!("Failed to load Save {}", error_msg)).unwrap();
            
        }
    }
}

fn updater_init(app_handle: AppHandle) {
    tauri::async_runtime::spawn(async move {
        println!("Checking for updates...");

        if let Err(e) = update(app_handle).await {
            println!("Update error: {:?}", e);
        } else {
            println!("Update check completed.")
        }
    });

}