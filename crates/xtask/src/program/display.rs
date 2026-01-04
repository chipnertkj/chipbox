use std::sync::Arc;

pub use self::pane::{ProgramOutput, ProgramOutputId, ProgramPane, ProgramPaneId};

mod pane;

pub struct ProgramDisplay {
    panes: Vec<ProgramPane>,
    outputs: Vec<ProgramOutput>,
}

impl ProgramDisplay {
    pub fn new() -> Self {
        Self::with_capacity(0, 0)
    }

    pub fn with_capacity(panes: usize, outputs: usize) -> Self {
        Self {
            panes: Vec::with_capacity(panes),
            outputs: Vec::with_capacity(outputs),
        }
    }

    pub fn add_output(&mut self, mut output: ProgramOutput) -> ProgramOutputId {
        let id = output.assign_id(self.outputs.len());
        self.outputs.push(output);
        id
    }

    pub fn create_output(
        &mut self,
        name: impl Into<Arc<str>>,
        pane_id: ProgramPaneId,
    ) -> ProgramOutputId {
        self.add_output(ProgramOutput::new(name, pane_id))
    }

    pub fn add_pane(&mut self, mut pane: ProgramPane) -> ProgramPaneId {
        let id = pane.assign_id(self.panes.len());
        self.panes.push(pane);
        id
    }

    pub fn create_pane(&mut self, name: impl Into<Arc<str>>) -> ProgramPaneId {
        self.add_pane(ProgramPane::new(name))
    }

    pub fn panes(&self) -> impl Iterator<Item = &ProgramPane> + '_ {
        self.panes.iter()
    }

    pub fn outputs(&self) -> impl Iterator<Item = &ProgramOutput> + '_ {
        self.outputs.iter()
    }
}
