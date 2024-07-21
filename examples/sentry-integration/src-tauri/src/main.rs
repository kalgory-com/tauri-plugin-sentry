// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sentry::TransactionContext;
use tauri_plugin_sentry::tracing::TransactionContextArg;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str, arg: TransactionContextArg) -> String {
    let context: Option<TransactionContext> = arg.into();
    let trace_info = match context {
        None => String::from("There is no distributed tracing header."),
        Some(context) => format!(
            "Distributed tracing with name: {} and trace-id: {}",
            context.name(),
            context.trace_id()
        ),
    };
    format!(
        "Hello, {}! You've been greeted from Rust!\n{}",
        name, trace_info
    )
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
