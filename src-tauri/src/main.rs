// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tokio::sync::Mutex;
pub mod arg;
pub mod rpc;
mod server;
use server::{Server,Item};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn search(info: &str, state: tauri::State<'_, Mutex<Server>>) -> Result<Vec<Item>, String> {
    state.lock().await.search(info).await
}

#[tauri::command]
async fn submit(
    obj_id: u32,
    state: tauri::State<'_, Mutex<Server>>,
) -> Result<(), String> {
    state.lock().await.submit(obj_id, None).await
}
#[tokio::main]
async fn main() {
    xlog_rs::log::init(std::io::stdout(), xlog_rs::log::Level::Trace);
    use clap::Parser;
    use tauri::Manager;
    let args = arg::Args::parse();
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                // window.open_devtools();
            }
            Ok(())
        })
        .manage(Mutex::new(
            Server::new(&args.uri)
                .await
                .unwrap(),
        ))
        .invoke_handler(tauri::generate_handler![search, submit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
