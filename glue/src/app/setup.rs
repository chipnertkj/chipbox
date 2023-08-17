#[cfg(feature = "backend")]
use chipbox_backend_lib as backend_lib;
use chipbox_common as common;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub enum Setup {
    #[default]
    First,
    Error(Rc<String>),
    Modify(Rc<common::Settings>),
}

#[cfg(feature = "backend")]
impl From<&backend_lib::Setup> for Setup {
    fn from(setup: &backend_lib::Setup) -> Self {
        match setup {
            backend_lib::Setup::First => Self::First,
            backend_lib::Setup::Error(error) => {
                Self::Error(Rc::new(error.to_string()))
            }
            backend_lib::Setup::Modify(settings) => {
                Self::Modify(Rc::new(settings.clone()))
            }
        }
    }
}
