// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
use tokio::runtime::Runtime;
use typesense::models::SearchResult;

fn main() {
    run()
}

#[tauri::command]
fn get_messages() -> String {
    let runtime = Runtime::new().unwrap();
    let typesense_configuration = core::search::get_typesense_configuration().unwrap();

    let result: SearchResult<core::search::Mail> =
        core::search::get_messages(&runtime, &typesense_configuration).unwrap();

    let result = serde_json::to_string(&result.hits).unwrap();

    return result;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_messages])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
