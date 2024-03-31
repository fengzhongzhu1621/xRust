use crate::platform::Signal;

#[derive(Debug)]
pub enum SignalType {
    /// Ctrl-C
    Ctrlc,
    /// Program termination
    /// Maps to `SIGTERM` and `SIGHUP` on *nix, `CTRL_CLOSE_EVENT` on Windows.
    Termination,
    /// Other signal/event using platform-specific data
    Other(Signal),
}
