#![feature(try_find)]
#![feature(const_for)]
#![feature(const_trait_impl)]

pub use chipbox_common as common;
pub use editor::audio_engine::stream_handle;
pub use editor::Editor;

use common::app::AwaitingConfig;
use tauri::{async_runtime, Manager as _};

pub mod editor;
mod error;
mod project_selection;
mod settings;

#[derive(Debug, PartialEq)]
pub enum ThreadMsg {
    /// A message from the frontend.
    Frontend(common::app::FrontendMsg),
    /// Immediatelly closes the app thread.
    Exit,
}

pub struct AppThread {
    app: App,
    tauri_app: tauri::AppHandle,
    rx: async_runtime::Receiver<ThreadMsg>,
}

pub enum App {
    /// Settings read has been attempted, but no valid configuration was found.
    AwaitingConfig(AwaitingConfig),
    Idle {
        settings: common::Settings,
    },
    Editor {
        inner: Box<Editor>,
        settings: common::Settings,
    },
}

impl From<&App> for common::app::State {
    fn from(app: &App) -> Self {
        match app {
            App::AwaitingConfig(state) => {
                common::app::State::AwaitingConfig(state.clone())
            }
            App::Idle { .. } => common::app::State::Idle,
            App::Editor { .. } => common::app::State::Editor,
        }
    }
}

impl AppThread {
    /// Creates a new instance of `Self`.
    ///
    /// Requires a `ThreadMessage` RX from a parent thread.
    ///
    /// Initial state is `App::Setup(Setup::First)`.
    pub fn new(
        rx: async_runtime::Receiver<ThreadMsg>,
        tauri_app: tauri::AppHandle,
    ) -> Self {
        AppThread {
            app: App::AwaitingConfig(AwaitingConfig::NoConfig),
            tauri_app,
            rx,
        }
    }

    /// Runs the app thread.
    ///
    /// Loads settings and enters the message loop.
    pub async fn run(mut self) {
        tracing::trace!("App thread started.");
        // Read settings.
        tracing::trace!("Reading settings.");
        let result = read_settings().await;
        // Handle the result.
        match result {
            Ok(after_read_settings) => {
                // Found config and converted to `AfterReadSettings`.
                tracing::trace!("Settings ok.");
                // Send message to client.
                self.send_message(common::app::BackendMsg::ReadingSettings);
                // Convert to `App`.
                self.app = after_read_settings.into();
                // Enter message loop.
                self.poll_messages().await
            }
            Err(e) => {
                // Something went wrong while reading settings.
                tracing::error!("Settings read failed: {}", e);
                // Poll for exit message.
                let _result =
                    Self::poll_message_until(&mut self.rx, |msg| match msg {
                        ThreadMsg::Exit => Some(()),
                        _ => None,
                    })
                    // We don't actually care about the result - we exit either way.
                    // If result is `Some(_)`, we received an exit message.
                    // If result is `None`, the channel has already been closed,
                    // which means RX will never receive another message.
                    .await;
            }
        }
        // Exit.
        tracing::trace!("App thread exiting.");
    }

    /// Send an `AppMessage` to the client window.
    fn send_message(&self, msg: common::app::BackendMsg) {
        tracing::trace!("Sending message to frontend: {:?}", msg);
        self.tauri_app
            .emit_all(common::app::BackendMsg::event_name(), msg)
            .expect("Failed to send message");
    }

    /// Polls messages from the `Receiver` in a loop.
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
        f: F,
    ) -> Option<T>
    where
        F: Fn(ThreadMsg) -> Option<T>,
    {
        tracing::trace!("Waiting for messages.");
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

    /// Polls messages from the `Receiver` in a loop.
    /// The messages are handled in a synchronous manner -
    /// their order is preserved.
    async fn poll_messages(&mut self) {
        #[allow(clippy::never_loop)]
        // Wait for messages, stop when the channel is closed.
        while let Some(msg) = self.rx.recv().await {
            tracing::trace!("App thread received message: {:?}", msg);
            let quit = self.handle_msg(msg).await;
            if quit {
                break;
            }
        }
        // Channel is closed.
        tracing::trace!("Channel was closed.");
    }

    /// Handles a message from the parent thread.
    /// Returns `true` if the app should quit.
    async fn handle_msg(&mut self, msg: ThreadMsg) -> bool {
        match msg {
            // Handle frontend message.
            ThreadMsg::Frontend(msg) => {
                self.handle_frontend_msg(msg)
                    .await;
            }
            // Quit.
            ThreadMsg::Exit => return true,
        };
        false
    }

    /// Handles a frontend message.
    async fn handle_frontend_msg(&mut self, msg: common::app::FrontendMsg) {
        match msg {
            common::app::FrontendMsg::QueryApp => self.send_message(
                common::app::BackendMsg::QueryAppResponse((&self.app).into()),
            ),
        }
    }
}

pub enum AfterReadSettings {
    ConfigOk { settings: common::Settings },
    NoConfig,
}

impl From<AfterReadSettings> for App {
    fn from(after_read_settings: AfterReadSettings) -> Self {
        match after_read_settings {
            AfterReadSettings::NoConfig => {
                Self::AwaitingConfig(AwaitingConfig::NoConfig)
            }
            AfterReadSettings::ConfigOk { settings } => Self::Idle { settings },
        }
    }
}

/// Read settings from the config file.
pub async fn read_settings() -> Result<AfterReadSettings, settings::Error> {
    use settings::SettingsExt as _;

    match common::Settings::read().await {
        // Settings found.
        Ok(settings) => Ok(AfterReadSettings::ConfigOk { settings }),
        // We catch `std::io::ErrorKind::NotFound`.
        Err(settings::Error::Io(ref e))
            if e.inner.kind() == std::io::ErrorKind::NotFound =>
        {
            // We return `Ok` because not having a config file is a valid state.
            Ok(AfterReadSettings::NoConfig)
        }
        // Something else went wrong.
        Err(e) => Err(e),
    }
}
