use super::{errno, Error, OsStr};

/// A type representing file descriptor on Unix.
pub type FileDesc = libc::c_int;

/// A type representing Process ID on Unix.
pub type Pid = libc::pid_t;

/// Returns the ID of the current process.
pub fn pid() -> Pid {
    unsafe { libc::getpid() }
}

/// Opens a file with only purpose of locking it. Creates it if it does not
/// exist. Path must not contain a nul-byte in the middle, but a nul-byte in the
/// end (and only in the end) is allowed, which in this case no extra allocation
/// will be made. Otherwise, an extra allocation is made.
pub fn open(path: &OsStr) -> Result<FileDesc, Error> {
    let fd = unsafe {
        libc::open(
            path.bytes.as_ptr(),
            libc::O_RDWR | libc::O_CLOEXEC | libc::O_CREAT,
            (libc::S_IRUSR | libc::S_IWUSR | libc::S_IRGRP | libc::S_IROTH)
                as libc::c_int,
        )
    };

    if fd >= 0 {
        Ok(fd)
    } else {
        Err(Error::last_os_error())
    }
}

/// Writes data into the given open file.
pub fn write(fd: FileDesc, mut bytes: &[u8]) -> Result<(), Error> {
    while bytes.len() > 0 {
        let written = unsafe {
            libc::write(fd, bytes.as_ptr() as *const libc::c_void, bytes.len())
        };
        if written < 0 && errno() != libc::EAGAIN {
            return Err(Error::last_os_error());
        }
        bytes = &bytes[written as usize..];
    }

    Ok(())
}

pub fn fsync(fd: FileDesc) -> Result<(), Error> {
    let result = unsafe { libc::fsync(fd) };

    if result >= 0 {
        Ok(())
    } else {
        Err(Error::last_os_error())
    }
}

/// Truncates the file referenced by the given file descriptor and seeks it to
/// the start.
pub fn truncate(fd: FileDesc) -> Result<(), Error> {
    let res = unsafe { libc::lseek(fd, 0, libc::SEEK_SET) };
    if res < 0 {
        return Err(Error::last_os_error());
    }

    let res = unsafe { libc::ftruncate(fd, 0) };
    if res < 0 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

/// Tries to lock a file and blocks until it is possible to lock.
pub fn lock(fd: FileDesc) -> Result<(), Error> {
    let res = unsafe { libc::flock(fd, libc::LOCK_EX) };
    if res >= 0 {
        Ok(())
    } else {
        Err(Error::last_os_error())
    }
}

/// Tries to lock a file but returns as soon as possible if already locked.
pub fn try_lock(fd: FileDesc) -> Result<bool, Error> {
    let res = unsafe { libc::flock(fd, libc::LOCK_EX | libc::LOCK_NB) };
    if res >= 0 {
        Ok(true)
    } else {
        let err = errno();
        if err == libc::EWOULDBLOCK || err == libc::EINTR {
            Ok(false)
        } else {
            Err(Error::from_raw_os_error(err as i32))
        }
    }
}

/// Unlocks the file.
pub fn unlock(fd: FileDesc) -> Result<(), Error> {
    let res = unsafe { libc::flock(fd, libc::LOCK_UN) };
    if res >= 0 {
        Ok(())
    } else {
        Err(Error::last_os_error())
    }
}

/// Closes the file.
pub fn close(fd: FileDesc) {
    unsafe { libc::close(fd) };
}
