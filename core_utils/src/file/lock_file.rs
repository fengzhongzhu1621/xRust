use crate::platform::{fmt, string::ToOsStr, sys};

#[derive(Debug)]
pub struct LockFile {
    locked: bool,
    desc: sys::FileDesc,
}

impl LockFile {
    /// Opens a file for locking, with OS-dependent locking behavior. On Unix,
    /// if the path is nul-terminated (ends with 0), no extra allocation will be
    /// made.
    pub fn open<P>(path: &P) -> Result<Self, sys::Error>
    where
        P: ToOsStr + ?Sized,
    {
        let path = path.to_os_str()?;
        // 打开文件返回句柄
        let desc = sys::open(path.as_ref())?;
        Ok(Self { locked: false, desc })
    }

    /// 对文件加锁
    pub fn lock(&mut self) -> Result<(), sys::Error> {
        if self.locked {
            // 不支持重复锁文件
            panic!("Cannot lock if already owning a lock");
        }
        // 对文件加锁
        sys::lock(self.desc)?;
        self.locked = true;
        Ok(())
    }

    pub fn unlock(&mut self) -> Result<(), sys::Error> {
        if !self.locked {
            panic!("Attempted to unlock already unlocked lockfile");
        }
        self.locked = false;
        sys::unlock(self.desc)?;
        // 解锁后清空文件内容
        sys::truncate(self.desc)?;
        Ok(())
    }

    pub fn lock_with_pid(&mut self) -> Result<(), sys::Error> {
        // 先锁住文件
        self.lock()?;

        // 然后在文件中写入进程id
        let result = writeln!(fmt::Writer(self.desc), "{}", sys::pid());
        if result.is_err() {
            let _ = self.unlock();
        }
        result
    }

    pub fn try_lock(&mut self) -> Result<bool, sys::Error> {
        if self.locked {
            panic!("Cannot lock if already owning a lock");
        }
        // 锁文件不阻塞
        let lock_result = sys::try_lock(self.desc);
        if let Ok(true) = lock_result {
            self.locked = true;
        }
        lock_result
    }

    pub fn try_lock_with_pid(&mut self) -> Result<bool, sys::Error> {
        match self.try_lock() {
            Ok(true) => (),
            Ok(false) => return Ok(false),
            Err(error) => return Err(error),
        }

        let result = sys::truncate(self.desc)
            .and_then(|_| writeln!(fmt::Writer(self.desc), "{}", sys::pid()));
        if result.is_err() {
            let _ = self.unlock();
        }
        result.map(|_| true)
    }

    pub fn owns_lock(&self) -> bool {
        self.locked
    }
}

impl Drop for LockFile {
    fn drop(&mut self) {
        if self.locked {
            let _ = self.unlock();
        }
        sys::close(self.desc);
    }
}

// Safe because:
// 1. We never actually access the contents of the pointer that represents the
// Windows Handle.
//
// 2. We require a mutable reference to actually mutate the file
// system.

/// 允许在线程间转移所有权
#[cfg(windows)]
unsafe impl Send for LockFile {}

/// 允许多线程访问
#[cfg(windows)]
unsafe impl Sync for LockFile {}
