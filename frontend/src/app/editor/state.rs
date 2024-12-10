use chipbox_common::project::latest::Project;

pub struct State {
    /// State of the project, should be an exact copy of the version
    /// on the backend.
    project: Project,
    /// Local changes to the project, unverified by the backend.
    project_preview: Project,
}

impl From<Project> for State {
    fn from(project: Project) -> Self {
        Self {
            project: project.clone(),
            project_preview: project,
        }
    }
}

impl State {
    pub fn project_preview(&self) -> &Project {
        &self.project_preview
    }
}
