use serde;
use tokio::sync::Mutex;
pub mod flag;
pub mod rpc;
pub fn extract_prompt(s: &str) -> Option<(String, String)> {
    if s.starts_with('|') {
        let mut iter = s[1..].splitn(2, '|');
        if let Some(prompt) = iter.next() {
            if let Some(content) = iter.next() {
                return Some((prompt.to_string(), content.to_string()));
            }
        }
        return None;
    }
    return Some((String::new(), s.to_string()));
}

#[derive(std::fmt::Debug, Clone, serde::Serialize)]
pub struct Item {
    pub obj_id: u32,
    pub name: String,
    pub arg_hint: Option<String>,
    pub icon: Option<String>,
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn search(
    info: &str,
    state: tauri::State<'_, Mutex<rpc::Server>>,
) -> Result<Vec<Item>, String> {
    let (prompt, content) = match extract_prompt(info) {
        Some((prompt, content)) => (prompt, content),
        None => {
            return Ok(vec![]);
        }
    };
    let mut binding = state.lock().await;
    if let Some(service) = binding.get_ext(&prompt).await {
        service
            .request(rpc::RequestSearch { content })
            .await
            .map_err(|e| e.to_string())
            .map(|mut dl| {
                dl.list
                    .drain(..)
                    .map(|d| Item {
                        obj_id: d.obj_id,
                        name: d.name,
                        arg_hint: d.hint,
                        icon: None,
                    })
                    .collect()
            })
    } else {
        Ok(vec![])
    }
}
#[tokio::main]
async fn main() {
    xlog_rs::log::init(std::io::stdout(), xlog_rs::log::Level::Trace);
    use clap::Parser;
    use tauri::Manager;
    let args = flag::Args::parse();
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                //   window.close_devtools();
            }
            Ok(())
        })
        .manage(Mutex::new(
            rpc::Server::connect(args.uri.parse::<tonic::transport::Endpoint>().unwrap())
                .await
                .unwrap(),
        ))
        .invoke_handler(tauri::generate_handler![search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
