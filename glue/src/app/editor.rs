#[cfg(feature = "backend")]
use chipbox_backend_lib as backend_lib;
use common::project::ProjectMeta;

use crate::ConfiguredState;
use chipbox_common as common;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Editor {
    settings: Rc<common::Settings>,
    project: Rc<common::Project>,
    project_meta: Rc<Option<ProjectMeta>>,
}

#[cfg(feature = "backend")]
impl From<&backend_lib::Editor> for Editor {
    fn from(editor: &backend_lib::Editor) -> Self {
        let backend_lib::Editor {
            settings,
            project,
            project_meta_opt: project_meta,
            ..
        } = editor;
        Self {
            settings: Rc::new(settings.clone()),
            project: Rc::new(project.clone()),
            project_meta: Rc::new(project_meta.clone()),
        }
    }
}

impl ConfiguredState for Editor {
    fn settings(&self) -> Rc<common::Settings> {
        self.settings.clone()
    }
}
