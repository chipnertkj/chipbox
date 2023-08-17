use crate::error;
use crate::settings::{self, SettingsExt as _};
use chipbox_common as common;

#[derive(Debug)]
pub enum Setup {
    First,
    Error(settings::Error),
    Modify(common::Settings),
}

impl Setup {
    pub async fn read_settings() -> Self {
        let result = common::Settings::read().await;
        match result {
            // If settings not found, return `Setup::First`.
            Err(settings::Error::Io(error::io::Error { e, .. }))
                if e.kind() == std::io::ErrorKind::NotFound =>
            {
                Setup::First
            }
            // On any other error, return `Setup::Error`.
            Err(e) => Setup::Error(e),
            // Otherwise, return `Setup::Modify`.
            Ok(settings) => Setup::Modify(settings),
        }
    }
}
