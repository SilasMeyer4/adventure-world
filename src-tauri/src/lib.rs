// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::sync::{Arc, Mutex};

use tauri::App;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use tauri_plugin_updater::UpdaterExt;

mod config;
mod database_manager;
mod helper;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let handle = app.handle().clone();
            database_init(app);
            config_init(app);
            updater_init(handle);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            database_manager::database::add_entity,
            database_manager::database::add_character,
            database_manager::database::add_item,
            database_manager::database::add_place,
            database_manager::database::add_encounter,
            database_manager::database::add_encounter_entity,
            database_manager::database::add_entity_item,
            database_manager::entities::insert_monster,
            database_manager::loot_groups::add_loot_group,
            database_manager::loot_groups::get_loot_group_by_id,
            database_manager::loot_groups::search_loot_group_by_name,
            config::manager::set_save_data_config,
            config::manager::set_settings_config,
            config::manager::save_configs,
            helper::path::get_exe_dir_path,
            helper::path::open_in_file_system
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        println!("Found update, downloading...");

        // Show a custom dialog with "Absolutely" and "Totally" buttons
        let response = app
            .dialog()
            .message("A new version of the app is available. Would you like to update now?")
            .title("Update Available")
            .buttons(MessageDialogButtons::OkCancelCustom(
                "Yes".to_string(),
                "No".to_string(),
            ))
            .blocking_show();

        if response {
            // User clicked "Absolutely" (button 1)
            // Proceed to download and install
            update
                .download_and_install(
                    |chunk, total| println!("Downloaded {} of {:?}", chunk, total),
                    || println!("Download complete"),
                )
                .await?;

            println!("Update installed!");
            app.restart(); // Restart the app after installation
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
    let resource_dir = app.path().app_data_dir();
    
    let resource_dir_path = resource_dir.unwrap(); // Unwrap once and store it


if let Some(resource_path) = resource_dir_path.as_path().to_str() {
    println!("Resource directory path: {}", resource_path);


    match database_manager::database::Database::load_or_create_database(resource_path, "database.dnd") {
        Ok(db) => {
            app.manage(db);
            app.emit("database_load", "Loaded Database Sucessfully")
                .unwrap(); //Does nothing?
            println!("Database loaded successfully");
        }

        Err(error) => {
            let error_msg = format!("Database failed to load: {}", error);
            println!("Error loading database: {}", error_msg);
            app.emit(
                "database_load",
                format!("Failed to load Database {}", error_msg),
            )
            .unwrap();
        }
    }
    }
}

fn config_init(app: &App) {
    let resource_dir = app.path().app_data_dir();
    
    let resource_dir_path = resource_dir.unwrap(); // Unwrap once and store it


    if let Some(resource_path) = resource_dir_path.as_path().to_str() {

        let config_manager = config::ConfigManager::new(resource_path); // Assuming you have a `new` method for creating the ConfigManager instance

        // Wrap the ConfigManager inside Arc and Mutex for thread safety
        let config_manager = Arc::new(Mutex::new(config_manager));

        // Set the ConfigManager as state for the app
        app.manage(config_manager);
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
