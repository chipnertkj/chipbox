//! Create a new project on the backend.

#[cfg(feature = "frontend")]
pub use frontend::Args;

#[cfg(feature = "backend")]
pub(crate) mod backend {
    use crate::loaded_project::LoadedProject;
    use chipbox_common::project::latest::{
        song::meta::{Author, SongMeta},
        Project,
    };

    /// Creates a project with the given song metadata and current timestamp.
    /// The project is stored in the [`LoadedProject`] state in the [tauri application](tauri::App).
    #[tauri::command(rename_all = "snake_case")]
    pub(crate) async fn create_project(
        state: tauri::State<'_, LoadedProject>,
        name: String,
        description: Option<String>,
        authors: Vec<Author>,
    ) -> Result<Project, !> {
        let project = Project::new(SongMeta::new_now(name, description, authors));
        let LoadedProject(mutex) = state.inner();
        *mutex.lock() = Some(project.clone());
        Ok(project)
    }
}

#[cfg(feature = "frontend")]
pub(crate) mod frontend {
    use crate::tauri_api::core::{invoke, InvokeResult};
    use chipbox_common::project::latest::{song::meta::Author, Project};

    /// Arguments for the [`create_project`] command.
    #[derive(serde::Serialize, Debug)]
    pub struct Args<'a> {
        /// The name of the project.
        pub name: &'a str,
        /// A short description of the project.
        pub description: Option<&'a str>,
        /// The authors of the project.
        pub authors: &'a [Author],
    }

    /// Request the backend to create a new project with the given arguments.
    /// Returns a copy of the project on success.
    pub async fn create_project<'a>(args: &'a Args<'a>) -> InvokeResult<'a, Project, Args<'a>> {
        // No need to unwrap `Result<_, !>` here, since serde seems to omit it.
        invoke("create_project", args).await
    }
}
