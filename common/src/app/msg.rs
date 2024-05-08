pub mod cmd;
pub mod request;

use self::cmd::BackendCmd;
use self::request::{BackendResponse, FrontendRequest};
use super::BackendAppState;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
/// Messages sent by the backend app thread.
pub enum BackendMsg {
    /// Send a command from the backend.
    Cmd(BackendCmd),
    /// Response to a frontend request.
    Response(BackendResponse),
}

impl BackendMsg {
    /// JS Event name used by the frontend.
    pub const fn event_name() -> &'static str {
        "chipbox-app-message"
    }
}

impl From<BackendCmd> for BackendMsg {
    fn from(cmd: BackendCmd) -> Self {
        Self::Cmd(cmd)
    }
}

impl From<BackendResponse> for BackendMsg {
    fn from(response: BackendResponse) -> Self {
        Self::Response(response)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
/// Messages sent by the frontend app thread.
pub enum FrontendMsg {
    /// Query information from the backend.
    Request(FrontendRequest),
}

impl From<FrontendRequest> for FrontendMsg {
    fn from(request: FrontendRequest) -> Self {
        Self::Request(request)
    }
}
