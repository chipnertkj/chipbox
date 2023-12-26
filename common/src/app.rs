use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum BackendMsg {
    ReadingSettings,
    QueryAppResponse(State),
}

impl BackendMsg {
    pub fn event_name() -> &'static str {
        "chipbox-app-message"
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum FrontendMsg {
    QueryApp,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub enum AwaitingConfig {
    #[default]
    /// It's the first time the application has been started.
    NoConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum State {
    ReadingSettings,
    AwaitingConfig(AwaitingConfig),
    Idle,
    Editor,
}
