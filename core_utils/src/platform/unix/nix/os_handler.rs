use super::{error::IOError, pipe::pipe2};
use crate::error::CtrlcError;
use nix::unistd;
use std::os::fd::BorrowedFd;
use std::os::unix::io::RawFd;

static mut PIPE: (RawFd, RawFd) = (-1, -1);

/// 默认的信号处理函数是向匿名管道发送一个字符
extern "C" fn os_handler(_: nix::libc::c_int) {
    // Assuming this always succeeds. Can't really handle errors in any meaningful way.
    unsafe {
        let fd = BorrowedFd::borrow_raw(PIPE.1);
        let _ = unistd::write(fd, &[0u8]); // 非阻塞
    }
}

/// Register os signal handler.
///
/// Must be called before calling [`block_ctrl_c()`](fn.block_ctrl_c.html)
/// and should only be called once.
///
/// # Errors
/// Will return an error if a system error occurred.
///
#[inline]
pub unsafe fn init_os_handler(overwrite: bool) -> Result<(), IOError> {
    use nix::fcntl;
    use nix::sys::signal;

    // 创建匿名管道，复制给全局变量
    PIPE = pipe2(fcntl::OFlag::O_CLOEXEC)?;

    let close_pipe = |e: nix::Error| -> IOError {
        // Try to close the pipes. close() should not fail,
        // but if it does, there isn't much we can do
        let _ = unistd::close(PIPE.1);
        let _ = unistd::close(PIPE.0);
        e
    };

    // 设置PIPE.1属性为非阻塞模式, PIPE.0为阻塞模式
    // Make sure we never block on write in the os handler.
    if let Err(e) = fcntl::fcntl(
        PIPE.1,
        fcntl::FcntlArg::F_SETFL(fcntl::OFlag::O_NONBLOCK),
    ) {
        return Err(close_pipe(e));
    }

    // 定义信号处理函数
    // SA_RESTART: 当信号处理函数返回后, 被该信号中断的系统 调用将自动恢复.
    let handler = signal::SigHandler::Handler(os_handler);
    #[cfg(not(target_os = "nto"))]
    let new_action = signal::SigAction::new(
        handler,
        signal::SaFlags::SA_RESTART, // 使被信号打断的syscall重新发起
        signal::SigSet::empty(),
    );
    // SA_RESTART is not supported on QNX Neutrino 7.1 and before
    #[cfg(target_os = "nto")]
    let new_action = signal::SigAction::new(
        handler,
        signal::SaFlags::empty(),
        signal::SigSet::empty(),
    );

    // 设置ctrl + c 的信号处理函数为 os_handler
    // 用户输入：比如在终端上按下组合键ctrl+C，产生SIGINT信号
    let sigint_old =
        match signal::sigaction(signal::Signal::SIGINT, &new_action) {
            Ok(old) => old,
            Err(e) => return Err(close_pipe(e)),
        };

    // SIG_DFL 是两种标准信号处理选项之一；
    // 它只会执行信号的默认函数。
    // 例如，在大多数系统上，对于 SIGQUIT 的默认操作是转储核心并退出，而对于 SIGCHLD 的默认操作是简单地忽略它。
    if !overwrite && sigint_old.handler() != signal::SigHandler::SigDfl {
        // 设置为默认信号处理函数
        signal::sigaction(signal::Signal::SIGINT, &sigint_old).unwrap();
        // 关闭匿名管道
        return Err(close_pipe(nix::Error::EEXIST));
    }

    #[cfg(feature = "termination")]
    {
        let sigterm_old =
            match signal::sigaction(signal::Signal::SIGTERM, &new_action) {
                Ok(old) => old,
                Err(e) => {
                    signal::sigaction(signal::Signal::SIGINT, &sigint_old)
                        .unwrap();
                    return Err(close_pipe(e));
                }
            };
        if !overwrite && sigterm_old.handler() != signal::SigHandler::SigDfl {
            signal::sigaction(signal::Signal::SIGINT, &sigint_old).unwrap();
            signal::sigaction(signal::Signal::SIGTERM, &sigterm_old).unwrap();
            return Err(close_pipe(nix::Error::EEXIST));
        }
        let sighup_old =
            match signal::sigaction(signal::Signal::SIGHUP, &new_action) {
                Ok(old) => old,
                Err(e) => {
                    signal::sigaction(signal::Signal::SIGINT, &sigint_old)
                        .unwrap();
                    signal::sigaction(signal::Signal::SIGTERM, &sigterm_old)
                        .unwrap();
                    return Err(close_pipe(e));
                }
            };
        if !overwrite && sighup_old.handler() != signal::SigHandler::SigDfl {
            signal::sigaction(signal::Signal::SIGINT, &sigint_old).unwrap();
            signal::sigaction(signal::Signal::SIGTERM, &sigterm_old).unwrap();
            signal::sigaction(signal::Signal::SIGHUP, &sighup_old).unwrap();
            return Err(close_pipe(nix::Error::EEXIST));
        }
    }

    Ok(())
}

/// Blocks until a Ctrl-C signal is received.
///
/// Must be called after calling [`init_os_handler()`](fn.init_os_handler.html).
///
/// # Errors
/// Will return an error if a system error occurred.
///
#[inline]
pub unsafe fn block_ctrl_c() -> Result<(), CtrlcError> {
    use std::io;
    let mut buf = [0u8];

    // TODO: Can we safely convert the pipe fd into a std::io::Read
    // with std::os::unix::io::FromRawFd, this would handle EINTR
    // and everything for us.
    loop {
        // 从匿名管道监听消息
        match unistd::read(PIPE.0, &mut buf[..]) {
            Ok(1) => break, // 读取一个字节
            Ok(_) => {
                return Err(CtrlcError::System(
                    io::ErrorKind::UnexpectedEof.into(),
                ))
            }
            Err(nix::errno::Errno::EINTR) => {}
            Err(e) => return Err(e.into()), // 调用 CtrlcError 的 From trait
        }
    }

    Ok(())
}
