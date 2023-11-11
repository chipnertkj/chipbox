#![feature(try_find)]

pub use configured_state::ConfiguredState;
pub use editor::Editor;
pub use project_selection::ProjectSelection;
pub use setup::Setup;

mod configured_state;
mod editor;
mod error;
mod project_selection;
mod settings;
mod setup;

pub type ManagedApp = std::sync::Arc<tokio::sync::Mutex<App>>;

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
