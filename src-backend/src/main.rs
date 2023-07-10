// Prevents additional console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(never_type)]

use std::time::Duration;

use anyhow::anyhow;
use chipbox_common::{AppState, Settings, SettingsLoadError};
use tauri::async_runtime::{self, Mutex};
use tauri::Manager;
use tokio::time::Instant;
use tracing_subscriber::util::SubscriberInitExt as _;

#[tauri::command]
async fn query_app_state(
    app_state_mx: tauri::State<'_, ManagedAppState>,
) -> Result<AppState, !> {
    const SLEEP_DURATION: Duration = Duration::from_millis(300);
    let instant_begin = Instant::now();
    loop {
        let app_state_opt = app_state_mx.lock().await;
        let elapsed = instant_begin.elapsed();
        match *app_state_opt {
            None => {
                tracing::info!(
                    "waiting for AppState initialization: {elapsed:?} elapsed"
                );
                tokio::time::sleep(SLEEP_DURATION).await
            }
            Some(ref app_state) => {
                tracing::info!("AppState query ok: {elapsed:?} elapsed");
                return Ok(app_state.clone());
            }
        }
    }
}

async fn load_settings() -> Result<Settings, SettingsLoadError> {
    let home_directory = home::home_dir()
        .ok_or_else(|| anyhow!("unable to get the user's home directory"))
        .map_err(|e| SettingsLoadError::NoHomeDirectory {
            inner: e.to_string(),
        })?;
    let path = Settings::file_path(&home_directory);
    let exists = tokio::fs::try_exists(&path)
        .await
        .map_err(|e| SettingsLoadError::IOError {
            inner: e.to_string(),
        })?;
    if !exists {
        return Err(SettingsLoadError::NotFound);
    }
    let data = tokio::fs::read_to_string(path)
        .await
        .map_err(|e| SettingsLoadError::IOError {
            inner: e.to_string(),
        })?;
    let settings = serde_json::from_str::<Settings>(&data).map_err(|e| {
        SettingsLoadError::DeserializationError {
            inner: e.to_string(),
        }
    })?;
    Ok(settings)
}

async fn init_app_state(app_handle: tauri::AppHandle) {
    let app_state_mx = app_handle.state::<ManagedAppState>();
    let mut app_state_opt = app_state_mx.lock().await;
    let settings_result = load_settings().await;
    let new_state = match settings_result {
        Ok(settings) => AppState::Home { settings },
        Err(e) => AppState::Setup {
            settings_result: Err(e),
        },
    };
    *app_state_opt = Some(new_state);
}

type ManagedAppState = Mutex<Option<AppState>>;

fn main() {
    tracing_subscriber::FmtSubscriber::default().init();
    tauri::Builder::default()
        .manage::<ManagedAppState>(Default::default())
        .setup(|app| {
            let runtime = async_runtime::handle();
            let app_handle = app.handle();
            runtime.spawn(async { init_app_state(app_handle).await });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![query_app_state])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
