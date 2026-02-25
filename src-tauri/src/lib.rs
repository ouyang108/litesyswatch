// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System}; // 必须包含 ProcessesToUpdate
use tauri::{App, Emitter}; // Emitter 用于发送事件
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn shadow_exit(){
    std::process::exit(0);
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            start_monitoring(app);
            Ok(())
        })
        // 监听窗口事件
        // .on_window_event(|_window, event| match event {
        //     tauri::WindowEvent::CloseRequested { .. } => {
        //         // 当点击关闭按钮时，直接强制退出整个进程
        //         std::process::exit(0);
        //     }
        //     _ => {}
        // })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet,shadow_exit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn start_monitoring(app: &App) {
    let app_handle = app.handle().clone();

    thread::spawn(move || {
        let mut sys = System::new_all();
        let cpu_count = sys.cpus().len() as f32;
        let mut counter = 0;

        // 【解决 Null 问题】将 Top 数据变量放在循环外，保持状态
        let mut last_top_mem = serde_json::Value::Null;
        let mut last_top_cpu = serde_json::Value::Null;

        loop {
            sys.refresh_cpu_usage();
            sys.refresh_memory();

            let global_cpu = sys.global_cpu_usage();
            let total_mem = sys.total_memory() as f64;
            let used_mem = sys.used_memory() as f64;
            let mem_percentage = if total_mem > 0.0 {
                (used_mem / total_mem) * 100.0
            } else {
                0.0
            };

            // 每 3 秒更新一次 Top 进程数据
            if counter % 3 == 0 {
                sys.refresh_processes_specifics(
                    ProcessesToUpdate::All,
                    true,
                    ProcessRefreshKind::nothing().with_cpu().with_memory(),
                );

                let mut mem_map: HashMap<String, u64> = HashMap::new();
                let mut cpu_map: HashMap<String, f32> = HashMap::new();

                for p in sys.processes().values() {
                    let name = p.name().to_string_lossy().into_owned();
                    if name == "Idle" || name.is_empty() {
                        continue;
                    }

                    // 累加内存与 CPU
                    *mem_map.entry(name.clone()).or_insert(0) += p.memory();
                    *cpu_map.entry(name).or_insert(0.0) += p.cpu_usage();
                }

                // 更新持久化的 Top 变量
                if let Some((name, bytes)) = mem_map.into_iter().max_by_key(|e| e.1) {
                    last_top_mem = serde_json::json!({
                        "name": name,
                        "value": format!("{:.1} MB", bytes as f64 / 1024.0 / 1024.0)
                    });
                }

                if let Some((name, total_cpu)) = cpu_map
                    .into_iter()
                    .filter(|e| e.0 != "System")
                    .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
                {
                    let usage = if cpu_count > 0.0 {
                        total_cpu / cpu_count
                    } else {
                        total_cpu
                    };
                    last_top_cpu =
                        serde_json::json!({ "name": name, "value": format!("{:.1}%", usage) });
                }
            }

            // 发送数据：此时 last_top_mem 会一直保留上一次计算的结果，不会出现 null
            let _ = app_handle.emit(
                "sys-status",
                serde_json::json!({
                    "cpu": format!("{:.1}", global_cpu),
                    "mem": format!("{:.1}", mem_percentage),
                    "mem_used": format!("{:.2} GB", used_mem / 1024.0 / 1024.0 / 1024.0),
                    "top_mem": last_top_mem,
                    "top_cpu": last_top_cpu,
                }),
            );

            counter = (counter + 1) % 60;
            thread::sleep(Duration::from_secs(1));
        }
    });
}
