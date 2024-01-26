// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_requests;

use crate::app_requests::run_app_request;
use core_lib::handle_request::run_core_request;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn handle_rpc_request(name: &str, options: &str) -> Option<String> {
  if let Ok(result) = run_core_request(name, options) {
    return Some(result);
  }
  if let Ok(result) = run_app_request(name, options) {
    return Some(result);
  }

  None
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      #[cfg(debug_assertions)]
      app.get_window("main").unwrap().open_devtools(); // `main` is the first window from tauri.conf.json without an explicit label
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![handle_rpc_request])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
