use tauri::Manager;
use std::time::Duration;
use log::info;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use std::sync::atomic::{AtomicUsize, Ordering};

static WINDOW_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[tauri::command]
fn frontend_ready(window: tauri::WebviewWindow, app: tauri::AppHandle) {
    println!("前端已就绪，当前窗口: {}", window.label());
    
    // 显示当前准备好的窗口
    if let Err(e) = window.show() {
        println!("显示窗口失败: {}", e);
    }
    
    // 销毁其他所有旧窗口，彻底释放内存
    let current_label = window.label();
    for (label, w) in app.webview_windows() {
        if label != current_label {
            println!("正在销毁旧窗口: {} 释放内存...", label);
            w.close().unwrap();
        }
    }
}

fn start_auto_reload(app_handle: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            
            let count = WINDOW_COUNTER.fetch_add(1, Ordering::SeqCst);
            let new_label = format!("main_{}", count);
            println!("后台定时器被触发，正在静默创建新窗口: {} ...", new_label);
            
            // 创建一个隐藏的新窗口
            let builder = tauri::WebviewWindowBuilder::new(
                &app_handle,
                &new_label,
                tauri::WebviewUrl::App(Default::default())
            )
            .visible(false); // 隐藏窗口，做到无感加载
            
            if let Err(e) = builder.build() {
                println!("创建新窗口失败: {}", e);
            } else {
                println!("新窗口 {} 正在后台加载...", new_label);
            }
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("=== RUST BACKEND STARTED (from lib.rs) ===");
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            start_auto_reload(app_handle);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, frontend_ready])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
