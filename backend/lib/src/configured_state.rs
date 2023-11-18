use chipbox_common as common;

pub trait ConfiguredState {
    fn settings(&self) -> &common::Settings;
    fn settings_mut(&mut self) -> &mut common::Settings;
}
