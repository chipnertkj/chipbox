use crate::ConfiguredState;
use chipbox_common as common;

#[derive(Default, Debug)]
pub struct Editor {
    pub settings: common::Settings,
}

impl ConfiguredState for Editor {
    fn settings(&self) -> &common::Settings {
        &self.settings
    }

    fn settings_mut(&mut self) -> &mut common::Settings {
        &mut self.settings
    }
}
