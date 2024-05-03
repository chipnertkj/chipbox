use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum BackendMsg {
    ReadingSettings,
    QueryAppResponse(State),
}

impl BackendMsg {
    pub const fn event_name() -> &'static str {
        "chipbox-app-message"
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum FrontendMsg {
    QueryApp,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub enum AwaitConfig {
    #[default]
    /// It's the first time the application has been started.
    NoConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum State {
    ReadingSettings,
    AwaitConfig(AwaitConfig),
    Idle,
    Editor,
}
