use crate::{filter, fmt, Env};

#[derive(Default)]
pub struct Builder {
    filter: filter::Builder,
    writer: fmt::writer::WriteBuilder,
    format: fmt::Builder,
    built: bool,
}

impl Builder {
    pub fn new() -> Builder {
        Default::default()
    }

    pub fn from_default_env() -> Self {
        Self::from_env(Env::default())
    }
}

/// Create a new builder with the default environment variables.
///
/// The builder can be configured before being initialized.
/// This is a convenient way of calling [`Builder::from_default_env`].
///
/// [`Builder::from_default_env`]: struct.Builder.html#method.from_default_env
pub fn builder() -> Builder {
    Builder::from_default_env()
}

pub fn from_env<'a, E>(env: E) -> Self
where
    E: Into<Env<'a>>,
{
    let mut builder = Builder::new();
    builder.parse_env(env);
    builder
}

pub fn parse_env<'a, E>(&mut self, env: E) -> &mut Self
where
    E: Into<Env<'a>>,
{
    let env = env.into();

    if let Some(s) = env.get_filter() {
        self.parse_filters(&s);
    }

    if let Some(s) = env.get_write_style() {
        self.parse_write_style(&s);
    }

    self
}
