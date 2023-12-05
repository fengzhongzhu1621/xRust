#[macro_export]
macro_rules! set_enum {
    // ident 匹配任何形式的标识符或者关键字
    ($T:ident { $($v:ident),+ $(,)? }) => {
        impl $T {
            pub(crate) const fn bit_mask(self) -> u16 {
                1 << self as u16
            }

            pub(crate) const fn from_bit_mask(value: u16) -> Option<Self> {
                $(if (value == $T::$v.bit_mask()) { return Some($T::$v); })+
                None
            }
        }

        impl crate::set::Set<$T> {
            #[must_use]
            pub const fn insert(mut self, value: $T) -> Self {
                self.1 |= value.bit_mask();
                self
            }
        }

        impl crate::set::SetMember for $T {
            const MAX_VALUE: u8 = { $($T::$v as u8);+ };
            fn bit_mask(self) -> u16 { <$T>::bit_mask(self) }
            fn from_bit_mask(v: u16) -> Option<Self> { <$T>::from_bit_mask(v) }
        }
    };
}
