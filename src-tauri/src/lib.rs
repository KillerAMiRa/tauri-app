use tauri::Manager;
use std::time::Duration;
// use log::info;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use std::sync::atomic::{AtomicBool, Ordering};

static IS_MAIN_ACTIVE: AtomicBool = AtomicBool::new(true);

// frontend_ready command removed

fn start_auto_reload(app_handle: tauri::AppHandle) {
    // 启动时创建一个常驻的副窗口，且默认隐藏
    if let Err(e) = tauri::WebviewWindowBuilder::new(
        &app_handle,
        "secondary",
        tauri::WebviewUrl::App(Default::default())
    )
    .visible(false)
    .build() {
        println!("创建副窗口失败: {}", e);
        return;
    }

    tauri::async_runtime::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(10)).await;
            
            let is_main = IS_MAIN_ACTIVE.load(Ordering::SeqCst);
            
            if let (Some(main_win), Some(sec_win)) = (
                app_handle.get_webview_window("main"),
                app_handle.get_webview_window("secondary")
            ) {
                if is_main {
                    // 当前是 main，准备后台刷新 secondary
                    println!("后台正在静默刷新 secondary 窗口...");
                    let _ = sec_win.eval("window.location.reload();");
                    
                    // 给足 2 秒加载时间
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    
                    println!("平滑切换：显示 secondary，隐藏 main");
                    let _ = sec_win.show();
                    let _ = main_win.hide();
                    
                    IS_MAIN_ACTIVE.store(false, Ordering::SeqCst);
                } else {
                    // 当前是 secondary，准备后台刷新 main
                    println!("后台正在静默刷新 main 窗口...");
                    let _ = main_win.eval("window.location.reload();");
                    
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    
                    println!("平滑切换：显示 main，隐藏 secondary");
                    let _ = main_win.show();
                    let _ = sec_win.hide();
                    
                    IS_MAIN_ACTIVE.store(true, Ordering::SeqCst);
                }
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
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
