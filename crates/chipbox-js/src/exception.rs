#[derive(Debug, thiserror::Error, Clone)]
#[error("{}", self.message())]
pub enum JsException {
    Exception {
        message: Option<String>,
        stack: Option<String>,
    },
    Value {
        message: String,
    },
}

impl JsException {
    pub fn from_js_exception(e: &rquickjs::Exception<'_>) -> Self {
        Self::Exception {
            message: e.message(),
            stack: e.stack(),
        }
    }

    pub fn from_js_value(value: &rquickjs::Value<'_>) -> Self {
        Self::Value {
            message: format!("js value thrown: {value:?}"),
        }
    }

    pub fn stack_trace(&self) -> Option<&str> {
        // TODO: this doesnt account for source maps... implement source mapping
        match self {
            Self::Exception { stack, .. } => stack.as_deref(),
            Self::Value { .. } => None,
        }
    }

    fn message(&self) -> &str {
        match self {
            Self::Exception { message, .. } => {
                message.as_deref().unwrap_or("(exception without message)")
            }
            Self::Value { message, .. } => message,
        }
    }
}
