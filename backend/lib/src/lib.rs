#![feature(try_find)]
#![feature(const_for)]
#![feature(const_trait_impl)]

pub use chipbox_common as common;
pub use editor::audio_engine::stream_handle;
pub use editor::Editor;

use common::app::AwaitConfigReason;
use settings::SettingsExt as _;
use tauri::{async_runtime, Manager as _};

pub mod editor;
mod error;
mod project_selection;
mod settings;

/// Messages received by the app thread.
#[derive(Debug, PartialEq)]
pub enum ThreadMsg {
    /// Tauri forwarded a message from the frontend.
    Frontend(common::app::FrontendMsg),
    /// Main requested to exit.
    /// Immediatelly closes the app thread.
    Exit,
}

/// App thread data.
pub struct AppThread {
    data: AppData,
    rx: async_runtime::Receiver<ThreadMsg>,
}

pub struct AppData {
    state: AppState,
    tauri_app: tauri::AppHandle,
}

/// Backend state.
#[derive(Default)]
pub enum AppState {
    #[default]
    ReadingSettings,
    /// Settings read has been attempted, but no valid configuration was found.
    AwaitConfig {
        reason: AwaitConfigReason,
    },
    Idle {
        settings: common::Settings,
    },
    Edit {
        inner: Box<Editor>,
        settings: common::Settings,
    },
}

impl From<Option<common::Settings>> for AppState {
    fn from(settings_opt: Option<common::Settings>) -> Self {
        match settings_opt {
            Some(settings) => AppState::Idle { settings },
            None => AppState::AwaitConfig {
                reason: AwaitConfigReason::NoConfig,
            },
        }
    }
}

/// ???
impl From<&AppState> for common::app::BackendAppState {
    fn from(app: &AppState) -> Self {
        match app {
            AppState::ReadingSettings => {
                common::app::BackendAppState::ReadingSettings
            }
            AppState::AwaitConfig { ref reason } => {
                common::app::BackendAppState::AwaitConfig { reason: *reason }
            }
            AppState::Idle { .. } => common::app::BackendAppState::Idle,
            AppState::Edit { .. } => common::app::BackendAppState::Editor,
        }
    }
}

impl AppThread {
    /// Create a new instance of `Self`.
    ///
    /// Requires a RX from the parent thread.
    pub fn new(
        rx: async_runtime::Receiver<ThreadMsg>,
        tauri_app: tauri::AppHandle,
    ) -> Self {
        let data = AppData {
            state: Default::default(),
            tauri_app,
        };

        AppThread { data, rx }
    }

    /// Run the app thread.
    ///
    /// Load settings and enter the message loop.
    pub async fn run(mut self) {
        tracing::trace!("App thread started.");

        // Read settings.
        tracing::trace!("Reading settings.");
        let result = read_settings().await;

        // Handle the result.
        match result {
            // Read attempt succeeded.
            Ok(settings_opt) => {
                tracing::trace!("Settings ok.");

                // Send message to client.
                Self::send_message(
                    &self.data.tauri_app,
                    common::app::BackendMsg::ReadingSettings,
                );

                // Update state based on whether there was a valid config.
                self.data.state = settings_opt.into();

                // Enter message loop.
                self.poll_messages().await
            }
            // Something went wrong while reading settings.
            Err(e) => {
                tracing::error!("Settings read failed: {}", e);

                // Wait for exit message.
                self.poll_until_exit_message()
                    .await;
            }
        }

        // Exit.
        tracing::trace!("App thread finished.");
    }

    // Wait for exit message.
    async fn poll_until_exit_message(&mut self) {
        // Await exit message.
        let result = Self::poll_message_until(&mut self.rx, |msg| match msg {
            ThreadMsg::Exit => Some(()),
            _ => None,
        })
        .await;

        // Handle result.
        match result {
            // All good.
            Some(_) => {
                tracing::trace!("Received exit message.");
            }
            // Channel was closed.
            None => {
                // Fail gracefully.
                tracing::error!(
                    "Channel was closed before the app thread received an exit message."
                );
            }
        }
    }

    /// Polls messages from the channel in a loop.
    /// The given closure is called to process the message.
    /// If the closure returns `None`, the next message is polled.
    /// If the closure returns `Some(T)`, the loop ends.
    ///
    /// # Returns
    /// - `Ok(T)` if the loop ends, with `T` being the return value of the closure.
    /// - `Err(())` if the channel is closed before the loop ends.
    ///
    /// # Notes
    /// - If you neither send the expected message or close the channel,
    /// the loop will never end.
    /// - If the closure never returns `Some(T)`, the loop will keep polling
    /// until the channel is closed.
    async fn poll_message_until<F, T>(
        rx: &mut async_runtime::Receiver<ThreadMsg>,
        mut f: F,
    ) -> Option<T>
    where
        F: FnMut(ThreadMsg) -> Option<T>,
    {
        tracing::trace!("Polling for messages.");
        // Wait for messages, stop when the channel is closed.
        while let Some(msg) = rx.recv().await {
            // Call the closure.
            let opt = f(msg);
            // Return `Ok(T)` if the closure returned `Some(T)`.
            // Otherwise, continue polling.
            if let Some(result) = opt {
                return Some(result);
            }
        }
        // Channel is closed.
        None
    }

    /// Polls messages from the channel in a loop.
    async fn poll_messages(&mut self) {
        Self::poll_message_until(&mut self.rx, |msg| {
            match self.data.handle_msg(msg) {
                true => Some(()),
                false => None,
            }
        })
        .await;
        // Channel was closed.
        tracing::trace!("Channel closed.");
    }

    /// Send an `AppMessage` to the client window.
    fn send_message(
        tauri_app: &tauri::AppHandle,
        msg: common::app::BackendMsg,
    ) {
        tracing::trace!("Sending message to frontend: {:?}", msg);
        tauri_app
            .emit_all(common::app::BackendMsg::event_name(), msg)
            .unwrap_or_else(|err| {
                tracing::error!("Failed to send message to frontend: {}", err);
            })
    }
}

impl AppData {
    /// Handles a message from the parent thread.
    /// Returns `true` if the app should quit.
    fn handle_msg(&mut self, msg: ThreadMsg) -> bool {
        match msg {
            // Handle frontend message.
            ThreadMsg::Frontend(msg) => self.handle_frontend_msg(msg),
            // Quit.
            ThreadMsg::Exit => return true,
        };
        false
    }

    /// Handles a frontend message.
    fn handle_frontend_msg(&mut self, msg: common::app::FrontendMsg) {
        match msg {
            common::app::FrontendMsg::Query(query) => match query {
                common::app::FrontendQuery::BackendAppState => {
                    AppThread::send_message(
                        &self.tauri_app,
                        common::app::BackendMsg::Response(
                            common::app::BackendResponse::BackendAppState(
                                (&self.state).into(),
                            ),
                        ),
                    )
                }
                common::app::FrontendQuery::Settings => {
                    let settings_opt = match self.state {
                        AppState::Idle { ref settings } => {
                            Some(settings.clone())
                        }
                        _ => None,
                    };
                    AppThread::send_message(
                        &self.tauri_app,
                        common::app::BackendMsg::Response(
                            common::app::BackendResponse::Settings(
                                settings_opt,
                            ),
                        ),
                    )
                }
            },
        }
    }
}

/// Read settings from the config file.
/// Returns `Ok(None)` if the config file does not exist.
pub async fn read_settings() -> Result<Option<common::Settings>, settings::Error>
{
    match common::Settings::read().await {
        // Settings found.
        Ok(settings) => Ok(Some(settings)),
        // We catch `std::io::ErrorKind::NotFound`.
        // Not having a config file is a valid state.
        Err(settings::Error::Io(ref e))
            if e.inner.kind() == std::io::ErrorKind::NotFound =>
        {
            Ok(None)
        }
        // Something else went wrong.
        Err(e) => Err(e),
    }
}
