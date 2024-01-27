// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_requests;

use core_lib::core_request::run_core_request;
use core_lib::dprintln;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn handle_core_request(name: &str, options: &str) -> Option<String> {
  match run_core_request(name, options) {
    Ok(result) => {
      return Some(result);
    }
    Err(e) => {
      dprintln!("Error running core request: {}", e);
    }
  }
  // match run_app_request(name, options) {
  //   Ok(result) => {
  //     return Some(result);
  //   }
  //   Err(e) => {
  //     dprintln!("Error running app request: {}", e);
  //   }
  // }

  None
}

// #[tauri::command]
// async fn handle_main_request(app_handle: tauri::AppHandle) {
//   println!("my custom command");
//   // app_handle.dialog().open().await;
// }

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      #[cfg(debug_assertions)]
      app.get_window("main").unwrap().open_devtools(); // `main` is the first window from tauri.conf.json without an explicit label
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      handle_core_request,
      // handle_main_request
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
