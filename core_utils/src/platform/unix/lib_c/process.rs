#![allow(bad_style)]
use libc::{self, c_int};

use super::unix_stream::{drain, notify};
use std::cmp;
use std::collections::HashMap;
use std::io;
use std::mem;
use std::os::unix::net::UnixStream;
use std::os::unix::prelude::*;
use std::process::{Child, ExitStatus};
use std::sync::{Mutex, Once};
use std::time::{Duration, Instant};

type StateMap = HashMap<*mut Child, (UnixStream, Option<ExitStatus>)>;

static INIT: Once = Once::new();
static mut STATE: *mut State = 0 as *mut _;

struct State {
    prev: libc::sigaction, // 备份的信号处理函数
    write: UnixStream,     // 写管道
    read: UnixStream,      // 读管道
    map: Mutex<StateMap>,
}

impl State {
    #[allow(unused_assignments)]
    fn init() {
        unsafe {
            // Create our "self pipe" and then set both ends to nonblocking
            // mode.
            // 创建双工非阻塞管道
            let (read, write) = UnixStream::pair().unwrap();
            read.set_nonblocking(true).unwrap();
            write.set_nonblocking(true).unwrap();

            let mut state = Box::new(State {
                prev: mem::zeroed(), // 0 填充对象
                write: write,
                read: read,
                map: Mutex::new(HashMap::new()), // 初始化加锁字典
            });

            // 注册子进程终止信号处理函数
            // 在 Unix 系统中，当一个子进程终止时，父进程会收到一个 SIGCHLD 信号。
            // SIGCHLD 是子进程状态改变时发送给父进程的信号。
            // 父进程可以捕获这个信号，并通过调用 wait() 或 waitpid() 等函数来获取子进程的退出状态、终止原因等信息。
            // Register our sigchld handler
            let mut new: libc::sigaction = mem::zeroed();
            new.sa_sigaction = sigchld_handler as usize;
            new.sa_flags =
                libc::SA_NOCLDSTOP | libc::SA_RESTART | libc::SA_SIGINFO;

            // 备份旧的信号处理函数
            assert_eq!(
                libc::sigaction(libc::SIGCHLD, &new, &mut state.prev),
                0
            );

            // 将Box堆 转换为 *mut State 指针
            STATE = mem::transmute(state);
        }
    }
}

impl State {
    fn wait_timeout(
        &self,
        child: &mut Child,
        dur: Duration,
    ) -> io::Result<Option<ExitStatus>> {
        // First up, prep our notification pipe which will tell us when our
        // child has been reaped (other threads may signal this pipe).
        let (read, write) = UnixStream::pair()?;
        read.set_nonblocking(true)?;
        write.set_nonblocking(true)?;

        // 判断进程是否执行完成，并创建子进程和 unix 管道的映射关系，子进程可写
        // Next, take a lock on the map of children currently waiting. Right
        // after this, **before** we add ourselves to the map, we check to see
        // if our child has actually already exited via a `try_wait`. If the
        // child has exited then we return immediately as we'll never otherwise
        // receive a SIGCHLD notification.
        //
        // If the wait reports the child is still running, however, we add
        // ourselves to the map and then block in `select` waiting for something
        // to happen.
        let mut map = self.map.lock().unwrap();
        if let Some(status) = child.try_wait()? {
            return Ok(Some(status));
        }
        // 因为此函数可以多次调用，每次调用都会将同一个子进程绑定到新的 write 管道上，替换掉之前的 write 管道
        // 因为 State 是全局变量，所以这里管理了所有子进程的超时上下文
        assert!(map.insert(child, (write, None)).is_none());
        drop(map); // 解锁

        // Make sure that no matter what when we exit our pointer is removed
        // from the map.
        struct Remove<'a> {
            state: &'a State,
            child: &'a mut Child,
        }
        impl<'a> Drop for Remove<'a> {
            fn drop(&mut self) {
                let mut map = self.state.map.lock().unwrap();
                drop(map.remove(&(self.child as *mut Child)));
            }
        }
        let remove = Remove { state: self, child };

        // Alright, we're guaranteed that we'll eventually get a SIGCHLD due
        // to our `try_wait` failing, and we're also guaranteed that we'll
        // get notified about this because we're in the map. Next up wait
        // for an event.
        //
        // Note that this happens in a loop for two reasons; we could
        // receive EINTR or we could pick up a SIGCHLD for other threads but not
        // actually be ready oureslves.
        let start = Instant::now();
        let mut fds = [
            libc::pollfd {
                fd: self.read.as_raw_fd(), // 特定的文件描述符，若设置为负值则忽略events字段并且revents字段返回0。
                events: libc::POLLIN, // 需要监视该文件描述符上的哪些事件。
                revents: 0, // poll函数返回时告知用户该文件描述符上的哪些事件已经就绪。
            },
            libc::pollfd {
                fd: read.as_raw_fd(),
                events: libc::POLLIN,
                revents: 0,
            },
        ];
        loop {
            let elapsed = start.elapsed();
            if elapsed >= dur {
                // 超时退出循环
                break;
            }

            // 计算剩余多少秒过期
            let timeout = dur - elapsed;
            let timeout = timeout
                .as_secs()
                .checked_mul(1_000) // 结果是否溢出
                .and_then(|amt| {
                    amt.checked_add(timeout.subsec_nanos() as u64 / 1_000_000)
                })
                .unwrap_or(u64::max_value());
            let timeout =
                cmp::min(<c_int>::max_value() as u64, timeout) as c_int;

            // 监视多个文件描述符上的事件是否就绪
            let r = unsafe { libc::poll(fds.as_mut_ptr(), 2, timeout) };
            let timeout = match r {
                0 => true,           // 如果timeout时间耗尽，则返回0。
                n if n > 0 => false, // 如果函数调用成功，则返回有事件就绪的文件描述符个数。
                n => {
                    // 如果函数调用失败，则返回-1，同时错误码会被设置。
                    let err = io::Error::last_os_error();
                    if err.kind() == io::ErrorKind::Interrupted {
                        continue;
                    } else {
                        panic!("error in select = {}: {}", n, err)
                    }
                }
            };

            // Now that something has happened, we need to process what actually
            // happened. There's are three reasons we could have woken up:
            //
            // 1. The file descriptor in our SIGCHLD handler was written to.
            //    This means that a SIGCHLD was received and we need to poll the
            //    entire list of waiting processes to figure out which ones
            //    actually exited.
            // 2. Our file descriptor was written to. This means that another
            //    thread reaped our child and listed the exit status in the
            //    local map.
            // 3. We timed out. This means we need to remove ourselves from the
            //    map and simply carry on.
            //
            // In the case that a SIGCHLD signal was received, we do that
            // processing and keep going. If our fd was written to or a timeout
            // was received then we break out of the loop and return from this
            // call.
            let mut map = self.map.lock().unwrap();

            // 从管道读取信号处理函数发送的数据，如果有数据说明父子进程退出了
            if drain(&self.read) {
                self.process_sigchlds(&mut map);
            }

            // 超时退出循环 或 通过向 State.map[child].write 写数据退出循环
            if drain(&read) || timeout {
                break;
            }
        }

        // 无用代码，因为 remove 实现了析构函数会自动释放 map 中的值，这里用于获取 ret 的值
        let mut map = self.map.lock().unwrap();
        let (_write, ret) = map.remove(&(remove.child as *mut Child)).unwrap();
        drop(map); // 无用代码

        Ok(ret)
    }

    fn process_sigchlds(&self, map: &mut StateMap) {
        for (&k, &mut (ref write, ref mut status)) in map {
            // Already reaped, nothing to do here
            if status.is_some() {
                // 已处理则忽略
                continue;
            }

            // 返回进程状态码，发送循环停止信号
            *status = unsafe { (*k).try_wait().unwrap() };
            if status.is_some() {
                notify(write);
            }
        }
    }
}

// Signal handler for SIGCHLD signals, must be async-signal-safe!
//
// This function will write to the writing half of the "self pipe" to wake
// up the helper thread if it's waiting. Note that this write must be
// nonblocking because if it blocks and the reader is the thread we
// interrupted, then we'll deadlock.
//
// When writing, if the write returns EWOULDBLOCK then we choose to ignore
// it. At that point we're guaranteed that there's something in the pipe
// which will wake up the other end at some point, so we just allow this
// signal to be coalesced with the pending signals on the pipe.
#[allow(unused_assignments)]
extern "C" fn sigchld_handler(
    signum: c_int,
    info: *mut libc::siginfo_t,
    ptr: *mut libc::c_void,
) {
    type FnSigaction =
        extern "C" fn(c_int, *mut libc::siginfo_t, *mut libc::c_void);
    type FnHandler = extern "C" fn(c_int);

    unsafe {
        // 向管道发送数据，由 drain(&self.read) 接受数据
        let state = &*STATE;
        notify(&state.write);

        // 恢复旧的信号处理函数
        let fnptr = state.prev.sa_sigaction;
        if fnptr == 0 {
            return;
        }
        if state.prev.sa_flags & libc::SA_SIGINFO == 0 {
            let action = mem::transmute::<usize, FnHandler>(fnptr);
            action(signum)
        } else {
            let action = mem::transmute::<usize, FnSigaction>(fnptr);
            action(signum, info, ptr)
        }
    }
}

pub fn wait_timeout(
    child: &mut Child,
    dur: Duration,
) -> io::Result<Option<ExitStatus>> {
    INIT.call_once(State::init);
    unsafe { (*STATE).wait_timeout(child, dur) }
}
