pub type JsModule = js_tracing_mod;

#[rquickjs::module]
#[allow(clippy::needless_pass_by_value, reason = "required by FromJsFunc")]
pub mod tracing_mod {

    #[rquickjs::function]
    pub fn trace(msg: String) {
        tracing::trace!(target: "js", "{msg}");
    }

    #[rquickjs::function]
    pub fn debug(msg: String) {
        tracing::debug!(target: "js", "{msg}");
    }

    #[rquickjs::function]
    pub fn info(msg: String) {
        tracing::info!(target: "js", "{msg}");
    }

    #[rquickjs::function]
    pub fn warn(msg: String) {
        tracing::warn!(target: "js", "{msg}");
    }

    #[rquickjs::function]
    pub fn error(msg: String) {
        tracing::error!(target: "js", "{msg}");
    }
}
