// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::thread;
use std::time::Duration;
use sysinfo::System;
use tauri::{Emitter, App}; // Emitter 用于发送事件
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default().setup(|app| {
        start_monitoring(app);
        Ok(())
    })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
fn start_monitoring(app: &App) {
    // 从 App 获取句柄并克隆，用于在闭包中使用
    let app_handle = app.handle().clone();

    thread::spawn(move || {
        let mut sys = System::new_all();
        loop {
            sys.refresh_cpu_usage();
            sys.refresh_memory();

            let cpu_usage = sys.global_cpu_usage();
            let total_mem = sys.total_memory() as f64;
            let used_mem = sys.used_memory() as f64;
            let mem_percentage = (used_mem / total_mem) * 100.0;
            let used_gb = used_mem / 1024.0 / 1024.0 / 1024.0;
            let _ = app_handle.emit("sys-status", serde_json::json!({
                "cpu": format!("{:.1}", cpu_usage),
                "mem": format!("{:.1}", mem_percentage),
                "mem_used": format!("{:.2} GB", used_gb)
            }));

            thread::sleep(Duration::from_secs(1));
        }
    });
}