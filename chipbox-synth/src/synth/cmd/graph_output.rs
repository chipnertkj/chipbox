use super::{GraphCmd, InputSlot, OutputSlot};

pub struct GraphOutputCmd;

impl GraphCmd for GraphOutputCmd {
    fn input_slots(&self) -> &[InputSlot] {
        &[]
    }

    fn output_slots(&self) -> &[OutputSlot] {
        &[]
    }
}
