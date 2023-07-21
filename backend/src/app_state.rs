//! Defines the possible application states in chipbox as an enumeration.

pub(crate) mod app_home; // ok
pub(crate) mod editor; // ok
pub(crate) mod first_time_setup; // ok
pub(crate) mod settings; // ok

use self::app_home::AppHome;
use self::editor::Editor;
use self::first_time_setup::FirstTimeSetup;
use self::settings::Settings;
use std::io;
use tauri::Manager;
use tokio::sync::Mutex;

/// Result type alias for this module's `Error` type.
pub(crate) type Result<T> = std::result::Result<T, Error>;

/// Errors encountered during the initialization of application state.
pub(crate) enum Error {
    Settings(settings::Error),
}

/// Enumeration of the possible application states.
#[derive(Debug, Default)]
pub(crate) enum AppState {
    /// Default initialization state.
    /// The task responsible for loading settings has not finished yet.
    ///
    /// Await until finished loading.
    #[default]
    LoadSettings,
    /// This is the first time the application was run on this machine.
    ///
    /// Take the user through the configuration process.
    FirstTimeSetup(FirstTimeSetup),
    /// Landing state for users that have been through the first-time configuration process.
    ///
    /// Select an existing `Project` or create a new one to continue to the editor.
    Home(AppHome),
    /// Main state of the program.
    ///
    /// Make modifications to the selected `Project`.
    Editor(Editor),
}

impl AppState {
    /// Select a variant based on user `Settings`:
    /// - If found, return `Self::Home`.
    /// - If not found, return `Self::FirstTimeSetup`.
    /// - On any other error, return `Error::Settings`.
    pub(crate) async fn from_user_config() -> Result<Self> {
        let result = Settings::read().await;
        match result {
            // Settings loading ok.
            Ok(settings) => {
                let state = Self::Home(AppHome::new(settings));
                Ok(state)
            }
            // Settings not found.
            Err(settings::Error::IO(e))
                if e.kind() == io::ErrorKind::NotFound =>
            {
                let state = Self::FirstTimeSetup(Default::default());
                Ok(state)
            }
            // Settings loading error.
            Err(e) => Err(Error::Settings(e)),
        }
    }
}

/// The application's `AppState` representation, as managed by `tauri::App`.
pub(crate) struct AppStateManaged {
    mx: Mutex<Result<AppState>>,
}

impl AppStateManaged {
    /// Initialize the inner `AppState` result in a `tauri::App`-managed `Self`.
    pub(crate) async fn setup(app: tauri::AppHandle) {
        let managed_state = app.state::<AppStateManaged>();
        let mut app_state_result = managed_state.mx.lock().await;
        let new_state_result = AppState::from_user_config().await;
        *app_state_result = new_state_result;
    }
}

impl Default for AppStateManaged {
    /// Constructs the managed state with an `Ok` value, containing the default `AppState` variant.
    fn default() -> Self {
        Self {
            mx: Mutex::new(Ok(Default::default())),
        }
    }
}
