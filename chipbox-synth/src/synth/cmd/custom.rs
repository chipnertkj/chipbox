use super::{ambassador_impl_GraphCmd, GraphCmd};
// The two imports below are required by ambassador_impl_GraphCmd.
use super::{InputSlot, OutputSlot};

#[derive(ambassador::Delegate, derive_more::From, derive_more::AsRef, derive_more::AsMut)]
#[delegate(GraphCmd, target = "0")]
pub struct CustomCmd<C: GraphCmd>(C);
