// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::Mutex;

struct SoldeState {
    solde: Mutex<f32>,
}

#[tauri::command]
fn get_solde(state: tauri::State<SoldeState>) -> f32 {
    let solde = state.solde.lock().unwrap();
    *solde
}

#[tauri::command]
fn augmenter_solde(state: tauri::State<SoldeState>) {
    let mut solde = state.solde.lock().unwrap();
    *solde += 1.0;
}

fn main() {
    tauri::Builder::default()
        .manage(SoldeState {
            solde: Mutex::new(0.0),
        })
        .invoke_handler(tauri::generate_handler![get_solde])
        .invoke_handler(tauri::generate_handler![augmenter_solde])
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
