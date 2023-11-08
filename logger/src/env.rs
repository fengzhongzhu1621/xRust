use super::Var;
use std::{borrow::Cow, cell::RefCell, env, io};

/// Set of environment variables to configure from.
#[derive(Debug)]
pub struct Env<'a> {
    filter: Var<'a>,
    write_style: Var<'a>,
}

impl<'a> Env<'a> {
    /// Get a default set of environment variables.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> Default for Env<'a> {
    fn default() -> Self {
        Env {
            filter: Var::new(DEFAULT_FILTER_ENV),
            write_style: Var::new(DEFAULT_WRITE_STYLE_ENV),
        }
    }
}
