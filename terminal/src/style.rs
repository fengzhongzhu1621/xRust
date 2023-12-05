use crate::{attr::Attribute, color::Color, quirk::Quirk};
use core_utils::{condition::Condition, set::Set};

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::{borrow::Cow, string::String};

#[cfg(feature = "std")]
use std::borrow::Cow;

#[derive(Default, Debug, Copy, Clone)]
pub struct Style {
    pub foreground: Option<Color>,
    pub background: Option<Color>,

    pub(crate) attributes: Set<Attribute>,
    pub(crate) quirks: Set<Quirk>,

    pub condition: Option<Condition>,
}

impl Style {
    const DEFAULT: Style = Style {
        foreground: None,
        background: None,
        attributes: Set::default(),
        quirks: Set::default(),
        condition: None,
    };

    #[inline]
    pub const fn new() -> Style {
        Style::DEFAULT
    }
}
