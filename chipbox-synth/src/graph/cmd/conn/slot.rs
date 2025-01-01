use crate::graph::CmdDataDesc;

#[derive(Clone, Debug, PartialEq)]
pub struct InputSlot<Desc: CmdDataDesc> {
    pub name: String,
    pub ty_descs: Vec<Desc>,
}

impl<Desc: CmdDataDesc> InputSlot<Desc> {
    pub fn new(name: impl Into<String>, ty_descs: impl IntoIterator<Item = Desc>) -> Self {
        Self {
            name: name.into(),
            ty_descs: ty_descs.into_iter().collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OutputSlot<Desc: CmdDataDesc> {
    pub name: String,
    pub ty_desc: Desc,
}

impl<Desc: CmdDataDesc> OutputSlot<Desc> {
    pub fn new(name: impl Into<String>, ty_desc: Desc) -> Self {
        Self {
            name: name.into(),
            ty_desc,
        }
    }

    pub fn connectable_to(&self, dest_slot: &InputSlot<Desc>) -> bool {
        dest_slot.ty_descs.iter().any(|dest| self.ty_desc == *dest)
    }
}
