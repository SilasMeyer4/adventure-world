// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use tauri_plugin_updater::UpdaterExt;

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
            tauri::async_runtime::spawn(async move {
                println!("Checking for updates...");

                if let Err(e) = update(handle).await {
                    println!("Update error: {:?}", e);
                } else {
                    println!("Update check completed.")
                }
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
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