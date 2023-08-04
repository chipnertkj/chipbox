//! Defines the possible application states in chipbox as an enumeration.

pub mod editor; // ok
pub mod settings;
pub mod welcome; // ok

pub use self::editor::{Editor, Project};
pub use self::settings::Settings;

use self::welcome::Welcome;
use serde::{Deserialize, Serialize};
use std::fmt;

#[cfg(feature = "backend")]
use std::io;
#[cfg(feature = "backend")]
use tauri::Manager;
#[cfg(feature = "backend")]
use tokio::sync::Mutex;

/// Result type alias for this module's `Error` type.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors encountered during the initialization of application state.
#[derive(Debug)]
pub enum Error {
    /// See inner type for more information.
    Settings(settings::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Settings(e) => write!(f, "unable to load app settings: {e}"),
        }
    }
}

impl std::error::Error for Error {}

/// Enumeration of the possible application states.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(tag = "type")]
pub enum State {
    /// Default initialization state.
    /// The task responsible for loading settings has not finished yet.
    ///
    /// Await until finished loading.
    #[default]
    LoadingSettings,
    /// This is the first time the application was run on this machine.
    ///
    /// Take the user through the configuration process.
    Setup,
    /// Landing state for users that have been through the first-time configuration process.
    ///
    /// Select an existing `Project` or create a new one to continue to the editor.
    Welcome(Welcome),
    /// Main state of the program.
    ///
    /// Make modifications to the selected `Project`.
    Editor(Editor),
}

#[cfg(feature = "backend")]
impl State {
    /// Select a variant based on user `Settings`:
    /// - If found, return `Self::Welcome`.
    /// - If not found, return `Self::Setup`.
    /// - On any other error, return `Error::Settings`.
    pub async fn from_user_config() -> Result<Self> {
        let result = Settings::read().await;
        match result {
            // Settings loading ok.
            Ok(settings) => {
                let state = Self::Welcome(Welcome::new(settings));
                Ok(state)
            }
            // Settings not found.
            Err(settings::Error::IO(e))
                if e.kind() == io::ErrorKind::NotFound =>
            {
                let state = Self::Setup;
                Ok(state)
            }
            // Settings loading error.
            Err(e) => Err(Error::Settings(e)),
        }
    }
}

#[cfg(feature = "backend")]
/// The application's `AppState` representation, as managed by `tauri::App`.
pub struct ManagedState {
    pub mx: Mutex<Result<State>>,
}

#[cfg(feature = "backend")]
impl ManagedState {
    /// Initialize the inner `AppState` result in a `tauri::App`-managed `Self`.
    pub async fn setup(app: tauri::AppHandle) {
        let managed_state = app.state::<ManagedState>();
        let mut app_state_result = managed_state.mx.lock().await;
        let new_state_result = State::from_user_config().await;
        *app_state_result = new_state_result;
    }
}

#[cfg(feature = "backend")]
impl Default for ManagedState {
    /// Constructs the managed state with an `Ok` value, containing the default `AppState` variant.
    fn default() -> Self {
        Self {
            mx: Mutex::new(Ok(Default::default())),
        }
    }
}
