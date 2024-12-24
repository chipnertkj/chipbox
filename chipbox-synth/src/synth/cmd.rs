pub mod conn;
pub mod custom;
pub mod graph_input;
pub mod graph_output;

pub use conn::{
    slot::{InputSlot, OutputSlot, SlotTy},
    Conn, ConnTyMismatch,
};
pub use custom::CustomCmd;
pub use graph_input::GraphInputCmd;
pub use graph_output::GraphOutputCmd;

#[enum_dispatch::enum_dispatch(GraphCmd)]
#[derive(enum_as_inner::EnumAsInner)]
pub enum Cmd<C: GraphCmd> {
    GraphInput(GraphInputCmd),
    GraphOutput(GraphOutputCmd),
    Custom(CustomCmd<C>),
}

impl<C: GraphCmd> Cmd<C> {
    pub fn input_slot(&self, slot: usize) -> Option<&InputSlot> {
        self.input_slots().get(slot)
    }
    pub fn output_slot(&self, slot: usize) -> Option<&OutputSlot> {
        self.output_slots().get(slot)
    }
    pub fn slot_ty_matches(src_slot: &OutputSlot, dest_slot: &InputSlot) -> bool {
        dest_slot.tys.contains(&src_slot.ty)
    }
}

#[ambassador::delegatable_trait]
#[enum_dispatch::enum_dispatch]
pub trait GraphCmd {
    fn input_slots(&self) -> &[InputSlot];
    fn output_slots(&self) -> &[OutputSlot];
}

impl GraphCmd for ! {
    fn input_slots(&self) -> &[InputSlot] {
        match *self {}
    }
    fn output_slots(&self) -> &[OutputSlot] {
        match *self {}
    }
}

impl GraphCmd for std::convert::Infallible {
    fn input_slots(&self) -> &[InputSlot] {
        match *self {}
    }
    fn output_slots(&self) -> &[OutputSlot] {
        match *self {}
    }
}
