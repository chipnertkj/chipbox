#[cfg(feature = "backend")]
use chipbox_backend_lib as backend_lib;

use crate::ConfiguredState;
use chipbox_common as common;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Home {
    Welcome(Rc<common::Settings>),
}

#[cfg(feature = "backend")]
impl From<&backend_lib::ProjectSelection> for Home {
    fn from(project_selection: &backend_lib::ProjectSelection) -> Self {
        Self::Welcome(Rc::new(
            project_selection
                .settings
                .clone(),
        ))
    }
}

impl ConfiguredState for Home {
    fn settings(&self) -> Rc<common::Settings> {
        match self {
            Home::Welcome(settings) => settings.clone(),
        }
    }
}
