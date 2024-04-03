use crate::error::CtrlcError;
use crate::platform;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::thread;

static INIT: AtomicBool = AtomicBool::new(false);
static INIT_LOCK: Mutex<()> = Mutex::new(());

pub fn set_handler<F>(user_handler: F) -> Result<(), CtrlcError>
where
    F: FnMut() + 'static + Send,
{
    init_and_set_handler(user_handler, true)
}

pub fn try_set_handler<F>(user_handler: F) -> Result<(), CtrlcError>
where
    F: FnMut() + 'static + Send,
{
    init_and_set_handler(user_handler, false)
}

fn init_and_set_handler<F>(
    user_handler: F,
    overwrite: bool,
) -> Result<(), CtrlcError>
where
    F: FnMut() + 'static + Send,
{
    // 原子读取数据
    if !INIT.load(Ordering::Acquire) {
        // 执行锁操作
        let _guard = INIT_LOCK.lock().unwrap();

        // 在加锁之前可能对INIT进行了修改，所以需要重新获取最新的值
        if !INIT.load(Ordering::Relaxed) {
            // 设置信号处理函数
            set_handler_inner(user_handler, overwrite)?;
            // 标记为设置成功
            INIT.store(true, Ordering::Release);

            return Ok(());
        }
    }

    // 多次调用初始化函数需要报错
    Err(CtrlcError::MultipleHandlers)
}

fn set_handler_inner<F>(
    mut user_handler: F,
    overwrite: bool,
) -> Result<(), CtrlcError>
where
    F: FnMut() + 'static + Send,
{
    // 注册信号处理函数
    unsafe {
        match platform::sys::init_os_handler(overwrite) {
            Ok(_) => {}
            Err(err) => {
                return Err(err.into());
            }
        }
    }

    // 启动一个线程
    thread::Builder::new()
        .name("ctrl-c".into())
        .spawn(move || loop {
            // 等待信号被触发
            unsafe {
                platform::sys::block_ctrl_c()
                    .expect("Critical system error while waiting for Ctrl-C");
            }
            // 执行用户自定义业务逻辑
            user_handler();
        })
        .expect("failed to spawn thread");

    Ok(())
}
