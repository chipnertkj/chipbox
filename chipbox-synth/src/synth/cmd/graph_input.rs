use super::{GraphCmd, InputSlot, OutputSlot};

pub struct GraphInputCmd;

impl GraphCmd for GraphInputCmd {
    fn input_slots(&self) -> &[InputSlot] {
        &[]
    }

    fn output_slots(&self) -> &[OutputSlot] {
        &[]
    }
}
