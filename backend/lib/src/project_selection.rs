use crate::ConfiguredState;
use chipbox_common as common;

#[derive(Debug)]
pub struct ProjectSelection {
    pub settings: common::Settings,
}

impl ConfiguredState for ProjectSelection {
    fn settings(&self) -> &common::Settings {
        &self.settings
    }

    fn settings_mut(&mut self) -> &mut common::Settings {
        &mut self.settings
    }
}
