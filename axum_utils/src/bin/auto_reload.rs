//! cargo install cargo-watch systemfd
//! systemfd --no-pid -s http::3000 -- cargo watch -x run
//! cargo run -p axum_utils --bin auto-reload

use axum::{response::Html, routing::get, Router};
use listenfd::ListenFd;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // ListenFd: 一个用于处理环境变量中的监听文件描述符（listen fd）的库，常用于容器化环境或某些部署场景
    // 从环境变量中创建一个ListenFd实例。这通常用于容器化环境，其中服务可能已经通过环境变量指定了监听的文件描述符。
    let mut listenfd = ListenFd::from_env();
    // 尝试从ListenFd实例中获取文件描述符为0的TCP监听器。如果成功，它将返回一个Some(listener)，否则返回None
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            // 将监听器设置为非阻塞模式。这是异步I/O操作所必需的
            listener.set_nonblocking(true).unwrap();
            // 将标准库的TCP监听器转换为Tokio的TCP监听器，以便与Axum和Tokio的异步运行时一起使用。
            TcpListener::from_std(listener).unwrap()
        }
        // otherwise fall back to local listening
        // 如果没有从环境变量中获取到TCP监听器，则代码将回退到本地监听
        None => TcpListener::bind("127.0.0.1:3000").await.unwrap(),
    };

    // run it
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
