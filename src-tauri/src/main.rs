// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, close_splashscreen])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 创建一个 Rust 命令
// 以上 Rust 代码的执行逻辑是创建一个 close_splashscreen 函数用来关闭启动视图并展示主视图，并将这个函数注册为一个 Rust 命令，在应用初始化时进行注册，以便在 JavaScript 中可以动态调用该命令。
#[tauri::command]
fn close_splashscreen(window: tauri::Window) {
    println!("{:#?}", window.get_window("splash"));

    // 关闭启动视图
    if let Some(splashscreen) = window.get_window("splash") {
        splashscreen.close().unwrap();
    }

    // 展示主视图
    window.get_window("main").unwrap().show().unwrap();
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
