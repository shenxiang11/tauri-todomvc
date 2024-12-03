use std::env;
use std::env::home_dir;
use std::fs::{create_dir_all, File};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct TodoItem {
    id: u64,
    title: String,
    completed: bool,
}

#[tauri::command]
fn save_data(data: Vec<TodoItem>) -> Result<(), String> {
    if let Some(home_dir) = env::home_dir() {
        let dir = home_dir.join(".tauri-todo");
        if !dir.exists() {
            let ret = create_dir_all(&dir);
            match ret {
                Ok(_) => (),
                Err(e) => return Err(format!("Failed to create dir: {:?}", e)),
            }
        }
        let file = dir.join("data.json");
        let ret = File::create(file);
        match ret {
            Ok(file) => {
                let ret = serde_json::to_writer(&file, &data);
                match ret {
                    Ok(_) => return Ok(()),
                    Err(e) => return Err(format!("Failed to write data: {:?}", e)),
                }
            }
            Err(e) => return Err(format!("Failed to create file: {:?}", e)),
        }
    }

    Err("Failed to load user home dir.".to_string())
}

#[tauri::command]
fn load_data() -> Result<Vec<TodoItem>, String> {
    let home_dir = home_dir().ok_or("Failed to load user home dir.")?;
    let dir = home_dir.join(".tauri-todo");
    if !dir.exists() {
        return Ok(vec!());
    }
    let file = dir.join("data.json");
    if !file.exists() {
        return Ok(vec!());
    }
    let ret = File::open(file);
    let file = match ret {
        Ok(file) => file,
        Err(e) => return Err(format!("Failed to open file: {:?}", e)),
    };

    let ret = serde_json::from_reader(file);
    match ret {
        Ok(data) => return Ok(data),
        Err(e) => return Err(format!("Failed to read data: {:?}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![save_data, load_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
