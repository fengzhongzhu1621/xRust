use crate::predicates::core::Styled;
use anstyle;
use std;

/// 定义 key，value 的样式
#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct Palette {
    key: anstyle::Style,
    value: anstyle::Style,
}

impl Palette {
    pub(crate) fn color() -> Self {
        if cfg!(feature = "color") {
            Self {
                key: anstyle::AnsiColor::Blue.on_default()
                    | anstyle::Effects::BOLD,
                value: anstyle::AnsiColor::Yellow.on_default()
                    | anstyle::Effects::BOLD,
            }
        } else {
            Self::plain()
        }
    }

    pub(crate) fn plain() -> Self {
        Self::default()
    }

    pub(crate) fn key<D: std::fmt::Display>(self, display: D) -> Styled<D> {
        Styled::new(display, self.key)
    }

    pub(crate) fn value<D: std::fmt::Display>(self, display: D) -> Styled<D> {
        Styled::new(display, self.value)
    }
}
