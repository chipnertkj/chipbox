mod config;
mod os_window;

use core::slice;
use std::{iter, vec};

pub use config::UiLayoutConfig;
pub use os_window::{Mode, OsWindow};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiLayout {
    pub main_os_window: OsWindow,
    pub other_os_windows: Vec<OsWindow>,
}

impl UiLayout {
    /// Always returns at least one window.
    pub fn os_windows(
        &self,
    ) -> iter::Chain<iter::Once<&OsWindow>, slice::Iter<'_, OsWindow>> {
        iter::once(&self.main_os_window).chain(self.other_os_windows.iter())
    }

    /// Always returns at least one window.
    pub fn os_windows_mut(
        &mut self,
    ) -> iter::Chain<iter::Once<&mut OsWindow>, slice::IterMut<'_, OsWindow>>
    {
        iter::once(&mut self.main_os_window).chain(
            self.other_os_windows
                .iter_mut(),
        )
    }
}

impl Default for UiLayout {
    fn default() -> Self {
        let main_os_window = Default::default();
        let other_os_windows = vec![];

        Self {
            main_os_window,
            other_os_windows,
        }
    }
}
