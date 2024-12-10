use super::PatternId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Timeline {
    slots: Vec<PatternId>,
}
