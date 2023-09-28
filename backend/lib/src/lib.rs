#![feature(async_fn_in_trait)]

pub use project_selection::ProjectSelection;
pub use setup::Setup;

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
    Editor,
}
