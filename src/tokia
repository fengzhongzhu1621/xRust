use tokio::{
    self,
    runtime::Runtime,
    time::{self, Duration},
};

/// 休眠n秒
async fn sleep(n: u64) -> u64 {
    time::sleep(Duration::from_secs(n)).await;
    n
}
