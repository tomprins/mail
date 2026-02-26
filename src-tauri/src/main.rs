// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
use tokio::runtime::Runtime;
use typesense::models::{SearchResult, SearchResultHit};

fn main() {
    run()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_mails])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_mails(query: String) -> Vec<SearchResultHit<core::search::Mail>> {
    println!("searching mails with query '{query}'");

    let runtime = Runtime::new().unwrap();
    let typesense_configuration = core::search::get_typesense_configuration().unwrap();

    let search_result: SearchResult<core::search::Mail> =
        core::search::get_mails(&runtime, &typesense_configuration, &query).unwrap();

    return search_result.hits.unwrap_or_default();
}
