use std::fmt::{Debug, Display};

mod conn;
pub use conn::{Conn, ConnTyMismatch, InputSlot, OutputSlot};

#[enum_delegate::register]
pub trait GraphCmd {
    type Data: CmdData;
    type Desc: CmdDataDesc;
    fn input_slots(&self) -> &[InputSlot<Self::Desc>];
    fn output_slots(&self) -> &[OutputSlot<Self::Desc>];
    fn render(&self, inputs: &[Option<Self::Data>]) -> Self::Data;
}

pub trait CmdData {}
pub trait CmdDataDesc: PartialEq + Debug + Display + Copy {}
