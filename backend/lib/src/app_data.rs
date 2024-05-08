pub mod msg;

mod app_state;
use self::app_state::AppState;

/// All data required for managing the application.
///
/// Includes its state and the handle to the Tauri application.
pub struct AppData {
    pub state: AppState,
    tauri_app: tauri::AppHandle,
}

impl AppData {
    /// Create new app data.
    pub fn new(tauri_app: tauri::AppHandle) -> Self {
        Self {
            state: Default::default(),
            tauri_app,
        }
    }

    /// Associated Tauri application handle.
    pub fn tauri_app(&self) -> &tauri::AppHandle {
        &self.tauri_app
    }
}
