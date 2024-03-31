use nix::unistd;
use std::os::fd::IntoRawFd;
use std::os::unix::io::RawFd;

/// 创建匿名管道并设置管道属性
/// pipe2(2) is not available on macOS, iOS, AIX or Haiku, so we need to use pipe(2) and fcntl(2)
#[inline]
#[cfg(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "haiku",
    target_os = "aix",
    target_os = "nto",
))]
pub fn pipe2(flags: nix::fcntl::OFlag) -> nix::Result<(RawFd, RawFd)> {
    use nix::fcntl::{fcntl, FcntlArg, FdFlag, OFlag};

    // 创建匿名管道
    let pipe = unistd::pipe()?;
    let pipe = (pipe.0.into_raw_fd(), pipe.1.into_raw_fd());

    let mut res = Ok(0);

    // 设置管道属性
    if flags.contains(OFlag::O_CLOEXEC) {
        // 在fork的子进程中用exec系列系统调用加载新的可执行程序之前，关闭子进程中fork得到的fd。
        res = res
            .and_then(|_| fcntl(pipe.0, FcntlArg::F_SETFD(FdFlag::FD_CLOEXEC)))
            .and_then(|_| {
                fcntl(pipe.1, FcntlArg::F_SETFD(FdFlag::FD_CLOEXEC))
            });
    }

    if flags.contains(OFlag::O_NONBLOCK) {
        // 读写非阻塞
        res = res
            .and_then(|_| fcntl(pipe.0, FcntlArg::F_SETFL(OFlag::O_NONBLOCK)))
            .and_then(|_| fcntl(pipe.1, FcntlArg::F_SETFL(OFlag::O_NONBLOCK)));
    }

    match res {
        Ok(_) => Ok(pipe),
        Err(e) => {
            // 设置管道属性失败则关闭管道
            let _ = unistd::close(pipe.0);
            let _ = unistd::close(pipe.1);
            Err(e)
        }
    }
}

#[inline]
#[cfg(not(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "haiku",
    target_os = "aix",
    target_os = "nto",
)))]
pub fn pipe2(flags: nix::fcntl::OFlag) -> nix::Result<(RawFd, RawFd)> {
    let pipe = unistd::pipe2(flags)?;
    Ok((pipe.0.into_raw_fd(), pipe.1.into_raw_fd()))
}
