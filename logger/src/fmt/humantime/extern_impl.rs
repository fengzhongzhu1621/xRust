use std::fmt;
use std::time::SystemTime;

use humantime::{
    format_rfc3339_micros, format_rfc3339_millis, format_rfc3339_nanos,
    format_rfc3339_seconds,
};

use crate::fmt::{Formatter, TimestampPrecision};

impl Formatter {
    /// Get a [`Timestamp`] for the current date and time in UTC.
    pub fn timestamp(&self) -> Timestamp {
        Timestamp {
            time: SystemTime::now(),
            precision: TimestampPrecision::Seconds,
        }
    }
}

/// An [RFC3339] formatted timestamp.
///
/// The timestamp implements [`Display`] and can be written to a [`Formatter`].
///
/// [RFC3339]: https://www.ietf.org/rfc/rfc3339.txt
/// [`Display`]: https://doc.rust-lang.org/stable/std/fmt/trait.Display.html
/// [`Formatter`]: struct.Formatter.html
pub struct Timestamp {
    time: SystemTime,
    precision: TimestampPrecision,
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatter = match self.precision {
            TimestampPrecision::Seconds => format_rfc3339_seconds,
            TimestampPrecision::Millis => format_rfc3339_millis,
            TimestampPrecision::Micros => format_rfc3339_micros,
            TimestampPrecision::Nanos => format_rfc3339_nanos,
        };

        formatter(self.time).fmt(f)
    }
}

impl fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /// A `Debug` wrapper for `Timestamp` that uses the `Display` implementation.
        struct TimestampValue<'a>(&'a Timestamp);

        impl<'a> fmt::Debug for TimestampValue<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::Display::fmt(&self.0, f)
            }
        }

        f.debug_tuple("Timestamp").field(&TimestampValue(self)).finish()
    }
}
