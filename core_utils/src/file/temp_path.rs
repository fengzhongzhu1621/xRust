use super::error::PathPersistError;
use super::imp;
use crate::error::IoResultExt;
use std::ffi::OsStr;
use std::mem;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::{fmt, fs, io};

pub struct TempPath {
    // 将一个值放在堆上而不是栈上。留在栈上的则是指向堆数据的指针。
    // path 的值通过 PathBuf.into_boxed_path() 获取
    pub(crate) path: Box<Path>,
}

impl fmt::Debug for TempPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.path.fmt(f)
    }
}

/// 资源释放时删除文件
impl Drop for TempPath {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

impl Deref for TempPath {
    type Target = Path;

    fn deref(&self) -> &Path {
        &self.path
    }
}

// TempPath -> &Path
impl AsRef<Path> for TempPath {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

// TempPath -> &OsStr
impl AsRef<OsStr> for TempPath {
    fn as_ref(&self) -> &OsStr {
        self.path.as_os_str()
    }
}

/// A path to a named temporary file without an open file handle.
///
/// This is useful when the temporary file needs to be used by a child process,
/// for example.
///
/// When dropped, the temporary file is deleted.
impl TempPath {
    /// Close and remove the temporary file.
    pub fn close(mut self) -> io::Result<()> {
        // &*self.path 将 Box<Path> -> &Path
        let result = fs::remove_file(&self.path).with_err_path(|| &*self.path);
        // 初始化 path 为空
        self.path = PathBuf::new().into_boxed_path();
        // Don't call `drop` 让 self 实例进入一个不能操作的状态，但是实例还在，可以下次调用。
        // forget 是调用了 mem crate 里面 ManuallyDrop::new() 。
        // ManuallyDrop 能够控制编译器，让其不要自动去调用实例的解构函数 (destructor)，
        // 这样就能保存之前函数运行的信息。forget 调用 ManuallDrop::new() ，
        // 就是把 self 实例创建成一个 ManuallyDrop，获得之前 Waker 的信息，
        // 这样在切换到另外一个runtime 或者 thread 的时候，之前的 Waker 信息就同步过去，不会随着进程结束而解构。
        //
        // 即回收变量占用的空间但从不关闭底层系统资源
        mem::forget(self);
        result
    }

    /// Persist the temporary file at the target path.
    pub fn persist<P: AsRef<Path>>(
        mut self,
        new_path: P,
    ) -> Result<(), PathPersistError> {
        match imp::persist(&self.path, new_path.as_ref(), true) {
            Ok(_) => {
                // Don't drop `self`. We don't want to try deleting the old
                // temporary file path. (It'll fail, but the failure is never
                // seen.)
                self.path = PathBuf::new().into_boxed_path();
                mem::forget(self);
                Ok(())
            }
            Err(e) => Err(PathPersistError { error: e, path: self }),
        }
    }

    /// Persist the temporary file at the target path if and only if no file exists there.
    /// 临时文件持久化
    pub fn persist_noclobber<P: AsRef<Path>>(
        mut self,
        new_path: P,
    ) -> Result<(), PathPersistError> {
        match imp::persist(&self.path, new_path.as_ref(), false) {
            Ok(_) => {
                // Don't drop `self`. We don't want to try deleting the old
                // temporary file path. (It'll fail, but the failure is never
                // seen.)
                self.path = PathBuf::new().into_boxed_path();
                mem::forget(self);
                Ok(())
            }
            Err(e) => Err(PathPersistError { error: e, path: self }),
        }
    }

    /// Keep the temporary file from being deleted. This function will turn the
    /// temporary file into a non-temporary file without moving it.
    pub fn keep(mut self) -> Result<PathBuf, PathPersistError> {
        match imp::keep(&self.path) {
            Ok(_) => {
                // Don't drop `self`. We don't want to try deleting the old
                // temporary file path. (It'll fail, but the failure is never
                // seen.)
                let path = mem::replace(
                    &mut self.path,
                    PathBuf::new().into_boxed_path(),
                );
                mem::forget(self);
                Ok(path.into())
            }
            Err(e) => Err(PathPersistError { error: e, path: self }),
        }
    }

    /// Create a new TempPath from an existing path. This can be done even if no
    /// file exists at the given path.
    /// 构造函数
    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into().into_boxed_path() }
    }
}
