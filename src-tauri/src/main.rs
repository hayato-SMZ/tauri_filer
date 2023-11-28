// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt::format;
use tauri::Manager;
use tauri::Window;
// filesystemWrapperを使う
mod filesystemWrapper;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn rootdir(path: &str) -> Vec<String> {
    let result = filesystemWrapper::fs::get_dir_entries(path.to_string());
    if result.is_err() {
        let errMessage = format!("{:?}", result.err().unwrap());
        let mut messages = Vec::new();
        messages.push(errMessage);
        return messages;
    } else {
        let resultItems = result.unwrap();
        // resultItemsをStringのVecに変換
        let mut resultItemsString = Vec::new();
        for item in resultItems {
            resultItemsString.push(format!("{:?}", item.file_name()));
        }
        return resultItemsString;
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
