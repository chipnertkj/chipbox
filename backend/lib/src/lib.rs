#![feature(try_find)]
#![feature(const_for)]
#![feature(const_trait_impl)]

pub use configured_state::ConfiguredState;
pub use editor::audio_engine::stream_handle;
pub use editor::Editor;
pub use project_selection::ProjectSelection;
pub use setup::Setup;

use std::sync::Arc;
use tokio::sync::Mutex;

pub mod editor;

mod configured_state;
mod error;
mod project_selection;
mod settings;
mod setup;

#[derive(Clone, Default)]
pub struct ManagedApp {
    pub arc: Arc<Mutex<App>>,
}

#[derive(Default)]
pub enum App {
    #[default]
    ReadingSettings,
    Setup(Setup),
    ProjectSelection(ProjectSelection),
    Editor(Box<Editor>),
}

impl App {
    pub fn as_configured_state(&self) -> Option<&dyn ConfiguredState> {
        match self {
            App::ReadingSettings => None,
            App::Setup(..) => None,
            App::ProjectSelection(project_selection) => Some(project_selection),
            App::Editor(editor) => Some(editor.as_ref()),
        }
    }

    pub fn as_configured_state_mut(
        &mut self,
    ) -> Option<&mut dyn ConfiguredState> {
        match self {
            App::ReadingSettings => None,
            App::Setup(..) => None,
            App::ProjectSelection(project_selection) => Some(project_selection),
            App::Editor(editor) => Some(editor.as_mut()),
        }
    }
}
