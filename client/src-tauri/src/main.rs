// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod config;
use config::Config;

#[tauri::command]
fn check_config_values(
    port: u16,
    url: String,
    login: String,
    gitlabtoken: String,
) -> Result<String, String> {
    match Config::check_values(port, url, login, gitlabtoken) {
        Ok(_) => Ok("".to_string()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

#[tauri::command]
fn ping() -> String {
    "pong".to_string()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![check_config_values, ping])
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
