use core_utils::set_enum;

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
pub enum Attribute {
    /// Makes text <b>bold</b>.
    ///
    /// Typically used via the [`bold()`](crate::Style::bold()) builder method.
    Bold,
    /// Makes text <span style="opacity: 50%">dim</span>.
    ///
    /// Typically used via the [`dim()`](crate::Style::dim()) builder method.
    Dim,
    /// Display text in <i>italics</i>.
    ///
    /// Typically used via the [`italic()`](crate::Style::italic()) builder
    /// method.
    Italic,
    /// <u>Underline</u> text.
    ///
    /// Typically used via the [`underline()`](crate::Style::underline())
    /// builder method.
    Underline,
    /// <style>@keyframes blinker { 50% { opacity: 0; } }</style>
    /// <span style="animation: blinker 1s linear infinite;">Blink.</span>
    ///
    /// Typically used via the [`blink()`](crate::Style::blink()) builder
    /// method.
    Blink,
    /// <style>@keyframes blinker { 50% { opacity: 0; } }</style>
    /// <span style="animation: blinker 0.5s linear infinite;">Blink rapidly.</span>
    ///
    /// Typically used via the [`rapid_blink()`](crate::Style::rapid_blink())
    /// builder method.
    RapidBlink,
    /// <span style="background: black; color: white;">Invert</span>
    /// (flip) the foreground and background colors.
    ///
    /// Typically used via the [`invert()`](crate::Style::invert()) builder
    /// method.
    Invert,
    /// <span style="color: #333; background: #000;">Conceal</span> text.
    ///
    /// Typically used via the [`conceal()`](crate::Style::conceal()) builder
    /// method.
    Conceal,
    /// Display text with a <s>strike</s> through it.
    ///
    /// Typically used via the [`strike()`](crate::Style::strike()) builder
    /// method.
    Strike,
}

impl Attribute {
    pub(crate) fn fmt(
        &self,
        f: &mut dyn core::fmt::Write,
    ) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Attribute::Bold => 1,
                Attribute::Dim => 2,
                Attribute::Italic => 3,
                Attribute::Underline => 4,
                Attribute::Blink => 5,
                Attribute::RapidBlink => 6,
                Attribute::Invert => 7,
                Attribute::Conceal => 8,
                Attribute::Strike => 9,
            }
        )
    }
}

set_enum! {
    Attribute { Bold, Dim, Italic, Underline, Blink, RapidBlink, Invert, Conceal, Strike }
}