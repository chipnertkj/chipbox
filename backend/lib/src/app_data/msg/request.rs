use crate::app_data::AppData;
use crate::{common, AppThread};
use common::app::msg::request::FrontendRequest;
use common::app::msg::BackendMsg;

mod backend_response;

/// Handles the request and sends a response to the frontend.
pub(super) fn handle_frontend_request(
    app_data: &mut AppData,
    request: FrontendRequest,
) {
    // Handle request and prepare response.
    let response = match request {
        FrontendRequest::AppState => backend_response::app_state(app_data),
        FrontendRequest::Settings => backend_response::settings(app_data),
        FrontendRequest::UseDefaultSettings => {
            backend_response::use_default_settings(app_data)
        }
    };

    // Send response.
    AppThread::send_message(
        &app_data.tauri_app,
        BackendMsg::Response(response),
    );
}
