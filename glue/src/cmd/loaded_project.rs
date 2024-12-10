//! Retrieve the currently loaded project.

#[cfg(feature = "backend")]
pub(crate) mod backend {
    use crate::loaded_project::LoadedProject;
    use chipbox_common::project::latest::Project;

    /// Return a copy of the currently loaded project state.
    #[tauri::command(rename_all = "snake_case")]
    pub(crate) async fn loaded_project(
        state: tauri::State<'_, LoadedProject>,
    ) -> Result<Option<Project>, !> {
        let LoadedProject(mutex) = state.inner();
        let loaded_project = mutex.lock().clone();
        tracing::info!("frontend requested current project state: {loaded_project:?}");
        Ok(loaded_project)
    }
}

#[cfg(feature = "frontend")]
pub(crate) mod frontend {
    use crate::tauri_api::core::{invoke, InvokeResult};
    use chipbox_common::project::latest::Project;

    /// Request the backend to retrieve a copy of the currently loaded project, if applicable.
    /// Returns `None` if no project is loaded.
    pub async fn loaded_project() -> InvokeResult<'static, Option<Project>, ()> {
        // No need to unwrap `Result<_, !>` here, since serde seems to omit it.
        invoke("loaded_project", &()).await
    }
}
