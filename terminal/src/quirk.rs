use core_utils::set_enum;

/// Enum representing a `yansi` quirk.
///
/// See the [crate level docs](crate#quirks) for details.
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
pub enum Quirk {
    /// Mask: omit when painting is disabled.
    ///
    /// Typically applied via the [`mask()`](crate::Painted::mask()) builder
    /// method.
    ///
    /// See the [crate level docs](crate#masking) for details.
    Mask,
    /// Wrap the value: replace resets with the wrapped styling.
    ///
    /// Typically applied via the [`wrap()`](crate::Painted::wrap()) builder
    /// method.
    ///
    /// See the [crate level docs](crate#wrapping) for details.
    Wrap,
    /// Linger: do not clear the style after it is applied.
    ///
    /// Typically applied via the [`linger()`](crate::Painted::linger()) builder
    /// method.
    ///
    /// See the [crate level docs](crate#lingering) for details.
    Linger,
    /// Always clear styling afterwards, even if no actual styling was applied.
    ///
    /// Overrides the [`Linger`](Quirk::Linger) quirk if present.
    ///
    /// Typically applied via the [`clear()`](crate::Painted::clear()) builder
    /// method.
    Clear,
    /// Brighten the foreground color if it is not already bright.
    ///
    /// Typically applied via the [`bright()`](crate::Painted::bright()) builder
    /// method.
    ///
    /// See the [crate level docs](crate#brightening) for details.
    Bright,
    /// Brighten the background color if it is not already bright.
    ///
    /// Typically applied via the [`on_bright()`](crate::Painted::on_bright())
    /// builder
    /// method.
    ///
    /// See the [crate level docs](crate#brightening) for details.
    OnBright,
}

set_enum! {
    Quirk { Mask, Wrap, Linger, Clear, Bright, OnBright }
}
