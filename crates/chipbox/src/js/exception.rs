#[derive(Debug, thiserror::Error, Clone)]
#[error("{}", self.display_message())]
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

    pub fn print_stack_trace(&self) {
        // TODO: this doesnt account for source maps... implement source mapping
        match self {
            Self::Exception { stack, .. } => {
                if let Some(stack) = stack {
                    eprintln!("Stack trace:\n{stack}");
                }
            }
            Self::Value { .. } => {}
        }
    }

    fn display_message(&self) -> &str {
        match self {
            Self::Exception { message, .. } => {
                message.as_deref().unwrap_or("(exception without message)")
            }
            Self::Value { message, .. } => message,
        }
    }
}
