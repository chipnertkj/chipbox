#[cfg(feature = "backend")]
use chipbox_backend_lib as backend_lib;

use crate::ConfiguredState;
use chipbox_common as common;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Editor {
    settings: Rc<common::Settings>,
}

#[cfg(feature = "backend")]
impl From<&backend_lib::Editor> for Editor {
    fn from(editor: &backend_lib::Editor) -> Self {
        Self {
            settings: Rc::new(editor.settings.clone()),
        }
    }
}

impl ConfiguredState for Editor {
    fn settings(&self) -> Rc<common::Settings> {
        self.settings.clone()
    }
}
