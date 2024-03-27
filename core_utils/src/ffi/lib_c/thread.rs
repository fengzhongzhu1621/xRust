/// A type representing Process ID on Unix.
pub type Pid = libc::pid_t;

/// Returns the ID of the current process.
pub fn pid() -> Pid {
    unsafe { libc::getpid() }
}
