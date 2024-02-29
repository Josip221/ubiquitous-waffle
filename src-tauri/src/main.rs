// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


#[tauri::command]
fn test_function(message: String) {
  println!("Hello from test function: {}", message);
}


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![test_function])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

