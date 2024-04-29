use anstyle;

#[derive(Debug)]
pub struct Styled<D> {
    display: D, // 可显示的对象
    style: anstyle::Style,
}

impl<D: std::fmt::Display> Styled<D> {
    pub fn new(display: D, style: anstyle::Style) -> Self {
        Self { display, style }
    }
}

impl<D: std::fmt::Display> std::fmt::Display for Styled<D> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            // 样式渲染
            write!(f, "{}", self.style.render())?; // 显示样式
            self.display.fmt(f)?; // 原始的内容
            write!(f, "{}", self.style.render_reset())?; // 样式重置
            Ok(())
        } else {
            self.display.fmt(f) // 原始的内容
        }
    }
}

/// 自定义调色板
#[derive(Copy, Clone, Debug, Default)]
pub struct Palette {
    description: anstyle::Style, // 定义描述的样式
    var: anstyle::Style,         // 变量的样式
    expected: anstyle::Style,    // 期望值的样式
}

impl Palette {
    pub fn new(alternate: bool) -> Self {
        if alternate && cfg!(feature = "color") {
            Self {
                description: anstyle::AnsiColor::Blue.on_default()
                    | anstyle::Effects::BOLD,
                var: anstyle::AnsiColor::Red.on_default()
                    | anstyle::Effects::BOLD,
                expected: anstyle::AnsiColor::Green.on_default()
                    | anstyle::Effects::BOLD,
            }
        } else {
            Self::plain()
        }
    }

    pub(crate) fn plain() -> Self {
        Self {
            description: Default::default(),
            var: Default::default(),
            expected: Default::default(),
        }
    }

    pub(crate) fn description<D: std::fmt::Display>(
        self,
        display: D,
    ) -> Styled<D> {
        Styled::new(display, self.description)
    }

    pub(crate) fn var<D: std::fmt::Display>(self, display: D) -> Styled<D> {
        Styled::new(display, self.var)
    }

    pub(crate) fn expected<D: std::fmt::Display>(
        self,
        display: D,
    ) -> Styled<D> {
        Styled::new(display, self.expected)
    }
}
