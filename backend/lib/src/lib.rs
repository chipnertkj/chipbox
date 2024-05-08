#![feature(try_find)]
#![feature(const_for)]
#![feature(const_trait_impl)]
#![feature(let_chains)]

pub use chipbox_common as common;
pub mod dir;

use app_data::{msg, AppData};
use common::app::msg::cmd::BackendCmd;
use common::app::msg::{BackendMsg, FrontendMsg};
use common::app::AwaitConfigReason;
use settings::SettingsExt as _;
use tauri::{async_runtime, Manager as _};
mod app_data;
mod error;
mod project_selection;
mod settings;

#[derive(Debug, PartialEq)]
/// Messages received by the app thread.
pub enum ThreadMsg {
    /// Tauri forwarded a message from the frontend.
    Frontend(FrontendMsg),
    /// Main requested to exit.
    /// Immediatelly closes the app thread.
    Exit,
}

/// App thread data.
///
/// Includes all relevant application data as well as a receiver for
/// incoming messages from main.
pub struct AppThread {
    data: AppData,
    rx: async_runtime::Receiver<ThreadMsg>,
}

impl AppThread {
    /// Create a new app thread, ready to run.
    ///
    /// Requires a channel RX from the parent thread.
    pub fn new(
        rx: async_runtime::Receiver<ThreadMsg>,
        tauri_app: tauri::AppHandle,
    ) -> Self {
        AppThread {
            data: AppData::new(tauri_app),
            rx,
        }
    }

    /// Run the app thread.
    ///
    /// Read settings and enter the message loop.
    pub async fn run(mut self) {
        tracing::trace!("App thread started.");

        // Read settings.
        tracing::trace!("Reading settings.");
        let result = Self::read_settings().await;

        // Handle the result.
        match result {
            // Read attempt succeeded.
            Ok(settings_opt) => {
                tracing::trace!("Settings ok.");

                // Prepare settings update message.
                let msg = BackendCmd::UpdateSettings(match settings_opt {
                    Some(ref settings) => Ok(settings.clone()),
                    None => Err(AwaitConfigReason::NoConfig),
                });

                // Send message to client.
                Self::send_message(self.data.tauri_app(), msg.into());

                // Update state based on whether there was a valid config.
                self.data.state = settings_opt.into();

                // Enter message loop.
                self.poll_messages().await
            }
            // Something went wrong while reading settings.
            Err(err) => {
                // Prepare error message.
                let err_msg = format!("Settings read failed: {}", err);
                tracing::error!(err_msg);

                // Send message to client.
                let msg = BackendCmd::UpdateSettings(Err(
                    AwaitConfigReason::Error(err_msg.into()),
                ));
                Self::send_message(self.data.tauri_app(), msg.into());

                // Wait for exit message.
                self.poll_until_exit_message()
                    .await;
            }
        };

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

        // Handle the result.
        match result {
            // Success.
            Some(_) => {
                tracing::trace!("Received exit message.");
            }
            // Channel is already closed.
            None => {
                // Fail gracefully.
                tracing::error!(
                    "Channel was closed before app thread received an exit message."
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
            match msg::handle_thread_msg(&mut self.data, msg) {
                true => Some(()),
                false => None,
            }
        })
        .await;
        // Channel was closed.
        tracing::trace!("Channel closed.");
    }

    /// Send an `AppMessage` to the client window.
    fn send_message(tauri_app: &tauri::AppHandle, msg: BackendMsg) {
        tracing::trace!("Sending message to frontend: {:?}", msg);
        tauri_app
            .emit_all(BackendMsg::event_name(), msg)
            .unwrap_or_else(|err| {
                tracing::error!("Failed to send message to frontend: {}", err);
            })
    }

    /// Read settings from the config file.
    /// Returns `Ok(None)` if the config file does not exist.
    async fn read_settings(
    ) -> Result<Option<common::Settings>, settings::SettingsError> {
        match common::Settings::read().await {
            // Settings found.
            Ok(settings) => Ok(Some(settings)),
            // We catch `std::io::ErrorKind::NotFound`.
            // Not having a config file is a valid state.
            Err(settings::SettingsError::Io(ref e))
                if e.err.kind() == std::io::ErrorKind::NotFound =>
            {
                Ok(None)
            }
            // Something else went wrong.
            Err(err) => Err(err),
        }
    }
}
