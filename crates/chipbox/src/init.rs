use miette::Context as _;

pub fn hooks(env_filter: tracing_subscriber::EnvFilter) -> miette::Result<()> {
    // Set up miette hook.
    miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .show_related_errors_as_nested()
                .with_cause_chain()
                .build(),
        )
    }))
    .wrap_err("set miette hook")?;
    // Set up tracing subscriber.
    tracing_subscriber::fmt::SubscriberBuilder::default()
        .with_max_level(tracing::Level::DEBUG)
        .with_env_filter(env_filter)
        .try_init()
        .map_err(|e| miette::miette!("{e}"))
        .wrap_err("set tracing subscriber")?;
    Ok(())
}

struct Directive {
    name: &'static str,
    level: tracing::Level,
}

pub struct TracingFilterBuilder {
    directives: Vec<Directive>,
}

impl TracingFilterBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            directives: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_self_directive(self, level: tracing::Level) -> Self {
        self.with_directive(env!("CARGO_PKG_NAME"), level)
    }

    #[must_use]
    pub fn with_directive(mut self, name: &'static str, level: tracing::Level) -> Self {
        self.directives.push(Directive { name, level });
        self
    }

    #[must_use]
    pub fn build_from_default_env(self) -> tracing_subscriber::EnvFilter {
        self.directives.into_iter().fold(
            tracing_subscriber::EnvFilter::from_default_env(),
            |env_filter, directive| env_filter.add_directive(directive.into()),
        )
    }
}

impl Default for TracingFilterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Directive> for tracing_subscriber::filter::Directive {
    fn from(directive: Directive) -> Self {
        format!("{}={}", directive.name, directive.level)
            .parse()
            .expect("valid env-filter directive")
    }
}
