use std::env;
use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io;
cfg_if::cfg_if! {
    if #[cfg(not(target_os = "wasi"))] {
        use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
    } else {
        #[cfg(feature = "nightly")]
        use std::os::wasi::fs::MetadataExt;
    }
}
use crate::file::util;
use std::path::Path;

use {
    rustix::fs::{rename, unlink},
    std::fs::hard_link,
};

/// 根据路径创建文件
pub fn create_named(
    path: &Path,
    open_options: &mut OpenOptions,
) -> io::Result<File> {
    open_options.read(true).write(true).create_new(true);

    #[cfg(not(target_os = "wasi"))]
    {
        open_options.mode(0o600); // 仅读写权限
    }

    open_options.open(path)
}

fn create_unlinked(path: &Path) -> io::Result<File> {
    let tmp;
    // shadow this to decrease the lifetime. It can't live longer than `tmp`.
    let mut path = path;
    if !path.is_absolute() {
        let cur_dir = env::current_dir()?;
        tmp = cur_dir.join(path);
        path = &tmp;
    }

    let f = create_named(path, &mut OpenOptions::new())?;
    // don't care whether the path has already been unlinked,
    // but perhaps there are some IO error conditions we should send up?
    let _ = fs::remove_file(path);
    Ok(f)
}

pub fn create(dir: &Path) -> io::Result<File> {
    use rustix::{fs::OFlags, io::Errno};
    OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(OFlags::TMPFILE.bits() as i32) // do not mix with `create_new(true)`
        .open(dir)
        .or_else(|e| {
            match Errno::from_io_error(&e) {
                // These are the three "not supported" error codes for O_TMPFILE.
                Some(Errno::OPNOTSUPP)
                | Some(Errno::ISDIR)
                | Some(Errno::NOENT) => create_unix(dir),
                _ => Err(e),
            }
        })
}

fn create_unix(dir: &Path) -> io::Result<File> {
    util::create_helper(
        dir,
        OsStr::new(".tmp"),
        OsStr::new(""),
        util::NUM_RAND_CHARS,
        |path| create_unlinked(&path),
    )
}

pub fn reopen(file: &File, path: &Path) -> io::Result<File> {
    let new_file = OpenOptions::new().read(true).write(true).open(path)?;
    let old_meta = file.metadata()?;
    let new_meta = new_file.metadata()?;
    if old_meta.dev() != new_meta.dev() || old_meta.ino() != new_meta.ino() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "original tempfile has been replaced",
        ));
    }
    Ok(new_file)
}

pub fn persist(
    old_path: &Path,
    new_path: &Path,
    overwrite: bool,
) -> io::Result<()> {
    if overwrite {
        rename(old_path, new_path)?;
    } else {
        // On Linux, use `renameat_with` to avoid overwriting an existing name,
        // if the kernel and the filesystem support it.
        #[cfg(any(target_os = "android", target_os = "linux"))]
        {
            use rustix::fs::{renameat_with, RenameFlags, CWD};
            use rustix::io::Errno;
            use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

            static NOSYS: AtomicBool = AtomicBool::new(false);
            if !NOSYS.load(Relaxed) {
                match renameat_with(
                    CWD,
                    old_path,
                    CWD,
                    new_path,
                    RenameFlags::NOREPLACE,
                ) {
                    Ok(()) => return Ok(()),
                    Err(Errno::NOSYS) => NOSYS.store(true, Relaxed),
                    Err(Errno::INVAL) => {}
                    Err(e) => return Err(e.into()),
                }
            }
        }

        // Otherwise use `hard_link` to create the new filesystem name, which
        // will fail if the name already exists, and then `unlink` to remove
        // the old name.
        hard_link(old_path, new_path)?;

        // Ignore unlink errors. Can we do better?
        let _ = unlink(old_path);
    }
    Ok(())
}

pub fn keep(_: &Path) -> io::Result<()> {
    Ok(())
}
