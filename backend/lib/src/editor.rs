use crate::ConfiguredState;
use chipbox_common as common;
use common::project::ProjectMeta;

#[derive(Default, Debug)]
pub struct Editor {
    pub settings: common::Settings,
    pub project: common::Project,
    pub project_meta_opt: Option<ProjectMeta>,
}

impl ConfiguredState for Editor {
    fn settings(&self) -> &common::Settings {
        &self.settings
    }

    fn settings_mut(&mut self) -> &mut common::Settings {
        &mut self.settings
    }
}
