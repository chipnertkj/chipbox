#![feature(async_fn_in_trait)]

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

#[derive(Default, Debug)]
pub enum App {
    #[default]
    ReadingSettings,
    Setup(Setup),
    ProjectSelection(ProjectSelection),
    Editor(Editor),
}

impl App {
    pub fn as_configured_state(&self) -> Option<&dyn ConfiguredState> {
        match self {
            App::ReadingSettings => None,
            App::Setup(..) => None,
            App::ProjectSelection(project_selection) => Some(project_selection),
            App::Editor(editor) => Some(editor),
        }
    }

    pub fn as_configured_state_mut(
        &mut self,
    ) -> Option<&mut dyn ConfiguredState> {
        match self {
            App::ReadingSettings => None,
            App::Setup(..) => None,
            App::ProjectSelection(project_selection) => Some(project_selection),
            App::Editor(editor) => Some(editor),
        }
    }
}
