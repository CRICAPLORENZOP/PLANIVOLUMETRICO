// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use std::sync::Mutex;

struct AppState {
    pending_file: Mutex<Option<String>>,
}

#[tauri::command]
fn get_opened_file(state: tauri::State<AppState>) -> Option<String> {
    let mut pending = state.pending_file.lock().unwrap();
    pending.take()
}

fn main() {
    let pending_file = Mutex::new(None);

    #[cfg(target_os = "windows")]
    {
        let mut files = Vec::new();
        for arg in std::env::args().skip(1) {
            if arg.starts_with('-') {
                continue;
            }
            let path = if arg.starts_with("file:///") {
                Some(PathBuf::from(arg.trim_start_matches("file:///")))
            } else if arg.starts_with("file://") {
                Some(PathBuf::from(arg.trim_start_matches("file://")))
            } else {
                Some(PathBuf::from(&arg))
            };
            if let Some(p) = path {
                if p.extension().map_or(false, |e| e.to_str() == Some("cml")) || p.extension().map_or(false, |e| e.to_str() == Some("json")) {
                    if let Ok(content) = std::fs::read_to_string(&p) {
                        files.push(content);
                    }
                }
            }
        }
        if let Some(content) = files.into_iter().next() {
            *pending_file.lock().unwrap() = Some(content);
        }
    }

    tauri::Builder::default()
        .manage(AppState { pending_file })
        .invoke_handler(tauri::generate_handler![get_opened_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
