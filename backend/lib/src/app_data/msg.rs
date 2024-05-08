use self::request::handle_frontend_request;

use super::AppData;
use crate::{common, ThreadMsg};
use common::app::msg::FrontendMsg;
mod request;

/// Handles a message from the parent thread.
/// Returns `true` if the app should quit.
pub fn handle_thread_msg(app_data: &mut AppData, msg: ThreadMsg) -> bool {
    match msg {
        // Handle frontend message.
        ThreadMsg::Frontend(msg) => handle_frontend_msg(app_data, msg),
        // Quit.
        ThreadMsg::Exit => return true,
    };
    false
}

/// Handles messages from the frontend.
fn handle_frontend_msg(app_data: &mut AppData, msg: FrontendMsg) {
    match msg {
        FrontendMsg::Request(request) => {
            handle_frontend_request(app_data, request)
        }
    }
}
