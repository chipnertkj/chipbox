use std::sync::Arc;

use self::task::TaskBuilder;
pub use self::{
    display::{ProgramDisplay, ProgramOutputId},
    task::{Task, TaskId},
};
use crate::command::Command;

mod display;
mod task;

pub struct ProgramCommand {
    cmd: Box<Command>,
    output: Option<ProgramOutputId>,
}

impl From<Command> for ProgramCommand {
    fn from(cmd: Command) -> Self {
        Self {
            cmd: Box::new(cmd),
            output: None,
        }
    }
}

impl ProgramCommand {
    pub fn output_to(mut self, output: ProgramOutputId) -> Self {
        assert!(self.output.is_none());
        self.output = Some(output);
        self
    }
}

pub struct ProgramMessage {
    msg: Arc<str>,
    output: ProgramOutputId,
}

impl ProgramMessage {
    pub fn new(msg: impl Into<Arc<str>>, output: ProgramOutputId) -> Self {
        Self {
            msg: msg.into(),
            output,
        }
    }
}

pub enum ProgramAction {
    Finish { abort: bool },
    ClearOutput(ProgramOutputId),
    Run(ProgramCommand),
    Message(ProgramMessage),
    WaitForSigint,
    CompilePortAudio(ProgramOutputId),
}

impl From<ProgramCommand> for ProgramAction {
    fn from(cmd: ProgramCommand) -> Self {
        ProgramAction::Run(cmd)
    }
}

impl From<ProgramMessage> for ProgramAction {
    fn from(msg: ProgramMessage) -> Self {
        ProgramAction::Message(msg)
    }
}

impl ProgramAction {
    pub fn finish() -> ProgramAction {
        ProgramAction::Finish { abort: false }
    }

    pub fn abort() -> ProgramAction {
        ProgramAction::Finish { abort: true }
    }
}

pub struct Program {
    name: Arc<str>,
    display: ProgramDisplay,
    tasks: Vec<Task>,
}

impl Program {
    pub fn new(name: impl Into<Arc<str>>, display: ProgramDisplay) -> Self {
        Self {
            name: name.into(),
            display,
            tasks: vec![],
        }
    }

    pub fn name(&self) -> Arc<str> {
        self.name.clone()
    }

    pub fn name_ref(&self) -> &str {
        self.name.as_ref()
    }

    pub fn display(&self) -> &ProgramDisplay {
        &self.display
    }

    pub fn tasks(&self) -> impl Iterator<Item = &Task> {
        self.tasks.iter()
    }

    pub fn add_task(&mut self, task: TaskBuilder) -> TaskId {
        let id = self.tasks.len();
        self.tasks.push(task.into());
        TaskId(id)
    }
}
