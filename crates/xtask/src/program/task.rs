//! TODO: it's hard to figure out when to start and stop a task
//! for example, what if a task has two dependencies as its start conditions (AfterAll)
//! and then one of them stops and starts and finishes? is that a restart for the task?
//! define properly.

use std::sync::Arc;

pub use self::builder::TaskBuilder;
use crate::program::ProgramAction;

mod builder;

/// A unique identifier for a task.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TaskId(pub(super) usize);

pub enum TaskCondition {
    /// Always false.
    Never,
    /// Always true.
    Always,
    /// True when all of the given conditions are true.
    All(Vec<TaskCondition>),
    /// True when any of the given conditions are true.
    Any(Vec<TaskCondition>),
    /// Negates the condition.
    Not(Box<TaskCondition>),
    /// True when the watcher for this crate has been ticked off.
    CrateChanged(Arc<str>),
    /// True when the watcher for this npm package has been ticked off.
    NpmPackageDefinitionChanged(Arc<str>),
    /// True if the task has finished without being aborted.
    TaskCompleted(TaskId),
    /// True if the task is currently running.
    TaskRunning(TaskId),
}

impl TaskCondition {
    pub fn negate(self) -> Self {
        Self::Not(Box::new(self))
    }

    pub fn all_tasks_completed(tasks: impl IntoIterator<Item = TaskId>) -> Self {
        Self::All(tasks.into_iter().map(Self::TaskCompleted).collect())
    }

    pub fn npm_pkg_def_changed(deps: impl IntoIterator<Item = impl Into<Arc<str>>>) -> Self {
        Self::Any(
            deps.into_iter()
                .map(|s| Self::NpmPackageDefinitionChanged(s.into()))
                .collect(),
        )
    }
}

pub struct Task {
    name: Option<Arc<str>>,
    wait_on: TaskCondition,
    restart_on: TaskCondition,
    abort_on: TaskCondition,
    on_start: Vec<ProgramAction>,
    on_abort: Vec<ProgramAction>,
    on_finish: Vec<ProgramAction>,
}

impl Task {
    pub fn builder() -> TaskBuilder {
        TaskBuilder::new()
    }
}

impl From<TaskBuilder> for Task {
    fn from(builder: TaskBuilder) -> Self {
        Self {
            name: builder.name,
            wait_on: builder.depend_on.unwrap_or(TaskCondition::Never),
            restart_on: builder.restart_on.unwrap_or(TaskCondition::Never),
            abort_on: builder.abort_on.unwrap_or(TaskCondition::Never),
            on_start: builder.on_start,
            on_abort: builder.on_abort,
            on_finish: builder.on_finish,
        }
    }
}
