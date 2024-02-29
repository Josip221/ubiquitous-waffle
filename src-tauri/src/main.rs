// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use serde::Serialize;



#[derive(Debug, Serialize)]
struct DirInfo {
    name: String,
    size: u64,
    is_dir: bool,
    is_file: bool,
    created: u64,
    modified: u64,
    // permissions: u32,

}

#[tauri::command]
fn get_current_dir() -> Vec<DirInfo> {
   let current_dir = std::env::current_dir().expect("Failed to get current directory");

     let mut dir_info_vec: Vec<DirInfo> = Vec::new();

    // List contents of the current directory
    if let Ok(entries) = fs::read_dir(&current_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                let metadata = fs::metadata(&path).expect("Failed to get metadata");

                let dir_info = DirInfo {
                    name: path.file_name().unwrap().to_string_lossy().into_owned(),
                    size: metadata.len(),
                    is_dir: metadata.is_dir(),
                    is_file: metadata.is_file(),
                    created: metadata.created().unwrap().elapsed().unwrap().as_secs(),
                    modified: metadata.modified().unwrap().elapsed().unwrap().as_secs(),
                    // permissions: metadata.permissions().mode(),
                };

                dir_info_vec.push(dir_info);
            }
        }
    } else {
        println!("Failed to read directory contents");
    }
    dir_info_vec
}


fn main() {
  // let menu = Menu::new();
    // .add_native_item(MenuItem::Quit);


  tauri::Builder::default()
    // .menu(menu)
    .invoke_handler(tauri::generate_handler![get_current_dir])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

