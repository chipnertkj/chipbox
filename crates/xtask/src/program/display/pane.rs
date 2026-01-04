use std::sync::Arc;

use crate::{
    logger::Logger,
    program::{ProgramAction, ProgramMessage},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProgramPaneId(usize);

pub struct ProgramPane {
    id: Option<ProgramPaneId>,
    name: Arc<str>,
}

impl ProgramPane {
    pub fn new(name: impl Into<Arc<str>>) -> Self {
        Self {
            id: None,
            name: name.into(),
        }
    }

    pub(super) fn assign_id(&mut self, self_id: usize) -> ProgramPaneId {
        assert!(self.id.is_none(), "pane id was already assigned");
        let id = ProgramPaneId(self_id);
        self.id = Some(id);
        id
    }

    pub fn name(&self) -> Arc<str> {
        self.name.clone()
    }

    pub fn name_ref(&self) -> &str {
        self.name.as_ref()
    }

    pub fn id(&self) -> Option<ProgramPaneId> {
        self.id
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProgramOutputId(usize);

impl ProgramOutputId {
    pub fn msg_action(self, msg: impl Into<Arc<str>>) -> ProgramAction {
        ProgramMessage::new(msg, self).into()
    }

    pub fn clear_action(self) -> ProgramAction {
        ProgramAction::ClearOutput(self)
    }
}

pub struct ProgramOutput {
    id: Option<ProgramOutputId>,
    pane_id: ProgramPaneId,
    name: Arc<str>,
    logger: Logger,
}

impl ProgramOutput {
    pub fn new(name: impl Into<Arc<str>>, pane_id: ProgramPaneId) -> Self {
        const ENTRIES: usize = 8192;
        const PARTIAL_BYTES: usize = 16 * 1024usize.pow(2);
        Self {
            id: None,
            pane_id,
            logger: Logger::new(ENTRIES, PARTIAL_BYTES),
            name: name.into(),
        }
    }

    pub(super) fn assign_id(&mut self, id: usize) -> ProgramOutputId {
        assert!(self.id.is_none(), "output id was already assigned");
        let id = ProgramOutputId(id);
        self.id = Some(id);
        id
    }

    pub fn name(&self) -> Arc<str> {
        self.name.clone()
    }

    pub fn name_ref(&self) -> &str {
        self.name.as_ref()
    }
}
