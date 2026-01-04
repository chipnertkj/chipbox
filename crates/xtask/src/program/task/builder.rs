use std::sync::Arc;

use super::{ProgramAction, TaskId};
use crate::program::{Program, task::TaskCondition};

#[must_use]
pub struct TaskBuilder {
    /// An optional display name for the task.
    pub name: Option<Arc<str>>,
    /// The condition this task will wait for before starting.
    /// The task will be aborted if the conditiion changes while running.
    pub depend_on: Option<TaskCondition>,
    /// The condition that has to be met for this task to restart.
    /// Restarting aborts the task if it's running.
    pub restart_on: Option<TaskCondition>,
    /// The condition that has to be met for this task to abort.
    /// Aborting stops a running task.
    pub abort_on: Option<TaskCondition>,
    /// Actions to run when the task starts.
    /// This includes restarts.
    pub on_start: Vec<ProgramAction>,
    /// Actions to run when the task aborts.
    /// This includes restarts.
    pub on_abort: Vec<ProgramAction>,
    /// Actions to run when the task runs to completion without being aborted.
    pub on_finish: Vec<ProgramAction>,
}

impl TaskBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            depend_on: None,
            restart_on: None,
            abort_on: None,
            on_start: vec![],
            on_abort: vec![],
            on_finish: vec![],
        }
    }

    pub fn depend_on_task(mut self, task: TaskId) -> Self {
        assert!(self.depend_on.is_none());
        self.depend_on = Some(TaskCondition::TaskCompleted(task));
        self
    }

    pub fn depend_on_tasks(mut self, tasks: impl IntoIterator<Item = TaskId>) -> Self {
        assert!(self.depend_on.is_none());
        self.depend_on = Some(TaskCondition::all_tasks_completed(tasks));
        self
    }

    pub fn watch_npm_pkg_defs(
        mut self,
        packages: impl IntoIterator<Item = impl Into<Arc<str>>>,
    ) -> Self {
        assert!(self.restart_on.is_none());
        self.restart_on = TaskCondition::npm_pkg_def_changed(packages).into();
        self
    }

    pub fn watch_crate(mut self, crate_name: impl Into<Arc<str>>) -> Self {
        assert!(self.restart_on.is_none());
        self.restart_on = TaskCondition::CrateChanged(crate_name.into()).into();
        self
    }

    pub fn abort_on_task_complete(mut self, task: TaskId) -> Self {
        assert!(self.abort_on.is_none());
        self.abort_on = Some(TaskCondition::TaskCompleted(task));
        self
    }

    pub fn add_to(self, program: &mut Program) -> TaskId {
        program.add_task(self)
    }
}

impl TaskBuilder {
    pub fn name(mut self, name: impl Into<Arc<str>>) -> Self {
        assert!(self.name.is_none());
        self.name = Some(name.into());
        self
    }

    pub fn depend_on(mut self, condition: TaskCondition) -> Self {
        assert!(self.depend_on.is_none());
        self.depend_on = Some(condition);
        self
    }

    pub fn restart_on(mut self, condition: TaskCondition) -> Self {
        assert!(self.restart_on.is_none());
        self.restart_on = Some(condition);
        self
    }

    pub fn abort_on(mut self, condition: TaskCondition) -> Self {
        assert!(self.abort_on.is_none());
        self.abort_on = Some(condition);
        self
    }

    pub fn on_start(mut self, actions: impl IntoIterator<Item = ProgramAction>) -> Self {
        self.on_start.extend(actions);
        self
    }

    pub fn on_abort(mut self, actions: impl IntoIterator<Item = ProgramAction>) -> Self {
        self.on_abort.extend(actions);
        self
    }

    pub fn on_finish(mut self, actions: impl IntoIterator<Item = ProgramAction>) -> Self {
        self.on_finish.extend(actions);
        self
    }
}
