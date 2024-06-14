#![feature(iter_array_chunks)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate core;

mod app;
mod ssh;

use app::commands::{get_interfaces, get_sessions, start_session};
use app::AppState;

use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            start_session,
            get_sessions,
            get_interfaces
        ])
        .manage(AppState::default())
        .run(tauri::generate_context!())
        .context("error while running tauri application")?;

    Ok(())
}
