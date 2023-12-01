// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use filesystem_wrapper::fs::DirEntryInfo;
use tauri::Manager;
mod filesystem_wrapper;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn rootdir(path: &str) -> Result<Vec<DirEntryInfo>, String> {
    let result = filesystem_wrapper::fs::get_dir_entries(path.to_string());
    println!("{:?}", result.is_ok());
    if result.is_err() {
        Err(result.err().unwrap())
    } else {
        Ok(result.unwrap())
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let main_window = app.get_window("main").unwrap();
                main_window.open_devtools();
                main_window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, rootdir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
