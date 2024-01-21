// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core_lib::handle_request::run_request;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn handle_rpc_request(name: &str, options: &str) -> Option<String> {
  if let Ok(result) = run_request(name, options) {
    return Some(result);
  }
  None
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![handle_rpc_request])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
