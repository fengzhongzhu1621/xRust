use windows::{core::Result, Win32::System::Threading::*};

// 全局对象
static COUNTER: std::sync::RwLock<i32> = std::sync::RwLock::new(0);

#[test]
fn test_create_thread_ppl_work() -> Result<()> {
    unsafe {
        // 创建新的工作对象。
        let work = CreateThreadpoolWork(Some(callback), None, None)?;

        // 将工作对象发布到线程池。 工作线程调用工作对象的回调函数。
        for _ in 0..10 {
            SubmitThreadpoolWork(work);
        }

        // 等待未完成的工作回调完成，并选择性地取消尚未开始执行的挂起回调。
        WaitForThreadpoolWorkCallbacks(work, false);
    }

    let counter = COUNTER.read().unwrap();
    println!("counter: {}", *counter);
    Ok(())
}

// 回调函数。 每次调用 SubmitThreadpoolWork 以发布工作对象时，工作线程都会调用此回调。
extern "system" fn callback(
    _: PTP_CALLBACK_INSTANCE,
    _: *mut std::ffi::c_void,
    _: PTP_WORK,
) {
    let mut counter = COUNTER.write().unwrap();
    *counter += 1;
}
