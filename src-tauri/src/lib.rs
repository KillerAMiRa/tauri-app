use tauri::Manager;
use std::time::Duration;
use log::info;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn start_auto_reload(app_handle: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            println!("后台定时器被触发，正在尝试获取 main 窗口...");
            if let Some(window) = app_handle.get_webview_window("main") {
                // 如果你想借鉴 Flutter 原生层面的“清空缓存”：
                // 注意：这个 API 会清空所有的网络缓存、Cookie 和 LocalStorage！
                // window.clear_all_browsing_data().unwrap();

                // 为了测试网页是否真的接收到了指令，我们让网页先变白，再强制跳转
                let js_code = r#"
                    document.body.style.transition = 'opacity 0.5s';
                    document.body.style.opacity = '0';
                    setTimeout(() => {
                        window.location.href = window.location.href;
                    }, 500);
                "#;

                if let Err(e) = window.eval(js_code) {
                    info!("定时重启 WebView 失败: {}", e);
                    println!("定时重启 WebView 失败: {}", e);
                } else {
                    println!("已向 WebView 发送硬刷新指令");
                }
            } else {
                println!("获取 main 窗口失败，可能窗口还没创建或者名字不对！");
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
