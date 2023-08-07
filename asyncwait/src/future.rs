use smol;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;
use std::task::Context;
use std::task::Poll;
use std::task::Waker;
use std::thread;
use std::time::Duration;

/// Shared state between the future and the waiting thread
/// 在Future和等待的线程间共享状态
struct SharedState {
    /// 定时(睡眠)是否结束
    completed: bool,
    /// 当睡眠结束后，线程可以用`waker`通知`TimerFuture`来唤醒任务
    waker: Option<Waker>,
}

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

impl TimerFuture {
    /// 构造函数
    pub fn new(duration: Duration) -> TimerFuture {
        // 创建一个需要在多线程间共享的对象
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();

            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                // 告诉 executor，应该唤醒的相关任务。当wake()被调用时，
                //  executor 知道与Waker相关联的任务是准备前进，并且，它的 Future 应再次进行 poll。
                waker.wake();
            }
        });

        TimerFuture { shared_state }
    }
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();

        if shared_state.completed {
            println!("TimerFuture completed");
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

pub fn timefuture_async() {
    // 创建future对象
    let tf = TimerFuture::new(Duration::from_millis(1000));
    // 等待异步任务完成
    smol::block_on(tf)
}
