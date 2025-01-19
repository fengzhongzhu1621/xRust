//! cargo run -p axum_utils --bin chat

use axum::{
    extract::{
        ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade},
        State,
    },
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Our shared state
struct AppState {
    // We require unique usernames. This tracks which usernames have been taken.
    user_set: Mutex<HashSet<String>>,
    // Channel used to send messages to all connected clients.
    tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    // 配置了日志记录系统。它首先尝试从环境变量中获取日志级别，如果失败，则默认设置为当前 crate 名称的 trace 级别。
    // 然后，它添加了一个格式化层，用于控制日志的输出格式，并初始化日志系统。
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| {
                    format!("{}=trace", env!("CARGO_CRATE_NAME")).into()
                }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Set up application state for use with with_state().
    // user_set 是一个互斥锁保护的 HashSet，用于存储已注册的用户名
    let user_set = Mutex::new(HashSet::new());

    // 创建了一个广播通道，用于向所有连接的客户端发送消息
    let (tx, _rx) = broadcast::channel(100);

    let app_state = Arc::new(AppState { user_set, tx });

    let app = Router::new()
        .route("/", get(index)) // 返回静态 html 页面
        .route("/websocket", get(websocket_handler))
        .with_state(app_state); // 将应用状态与路由器关联起来

    let listener =
        tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn websocket_handler(
    ws: WebSocketUpgrade, // WebSocketUpgrade 类型，表示一个 WebSocket 升级请求。这是 axum 提供的一个类型，用于处理从 HTTP 到 WebSocket 的转换。
    State(state): State<Arc<AppState>>, // 表示应用的状态。这里使用了 Arc（原子引用计数）来共享状态，以便多个线程或异步任务可以安全地访问它。
) -> impl IntoResponse {
    // 将 HTTP 请求转换为 WebSocket 连接，并调用提供的闭包来处理这个连接
    ws.on_upgrade(|socket| websocket(socket, state))
}

// websocket 函数负责处理具体的 WebSocket 连接。它接收一个 WebSocket 对象和一个 State 对象作为参数。
// 一个自定义的异步函数，用于处理 WebSocket 连接。它接收一个 WebSocket 对象和一个 State 对虫对象作为参数。
// This function deals with a single websocket connection, i.e., a single
// connected client / user, for which we will spawn two independent tasks (for
// receiving / sending chat messages).
async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    // By splitting, we can send and receive at the same time.
    // 将 WebSocket 对象分割成发送和接收两个部分，以便同时处理发送和接收消息。
    let (mut sender, mut receiver) = stream.split();

    // 处理 websocket.onopen
    // Username gets set in the receive loop, if it's valid.
    let mut username = String::new();
    // Loop until a text message is found.
    while let Some(Ok(message)) = receiver.next().await {
        // 循环接收消息，直到收到一个文本消息
        if let Message::Text(name) = message {
            // If username that is sent by client is not taken, fill username string.
            // 检查用户名是否已被使用。如果未被使用，则将其添加到用户集合中，并将其保存到 string 变量中。
            check_username(&state, &mut username, name.as_str());

            // If not empty we want to quit the loop else we want to quit function.
            if !username.is_empty() {
                break;
            } else {
                // Only send our client that username is taken.
                let _ = sender
                    .send(Message::Text(Utf8Bytes::from_static(
                        "Username already taken.",
                    )))
                    .await;

                return;
            }
        }
    }

    // 订阅广播通道，以便接收其他客户端发送的消息。
    // We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client.
    let mut rx = state.tx.subscribe();

    // 向所有订阅者发送 "joined" 消息，表示新用户已加入。
    // Now send the "joined" message to all subscribers.
    let msg = format!("{username} joined.");
    tracing::debug!("{msg}");
    let _ = state.tx.send(msg); // 广播

    // 创建一个异步任务，用于接收广播消息并通过 WebSocket 发送给客户端。
    // Spawn the first task that will receive broadcast messages and send text
    // messages over the websocket to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::text(msg)).await.is_err() {
                // 广播失败则说明客户端断开连接，停止广播监听任务
                break;
            }
        }
    });

    // Clone things we want to pass (move) to the receiving task.
    let tx = state.tx.clone();
    let name = username.clone();

    // 创建另一个异步任务，用于接收客户端发送的消息，并将其广播给所有订阅者。
    // Spawn a task that takes messages from the websocket, prepends the user
    // name, and sends them to all broadcast subscribers.
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // Add username before message.
            let _ = tx.send(format!("{name}: {text}")); // 广播
        }
    });

    // 用于同时等待多个异步操作，并在其中一个操作完成时执行相应的处理逻辑
    // 会阻塞当前任务，直到其中一个分支的条件满足。
    // 使用 tokio::select 定义任务协调逻辑。如果其中一个任务完成（例如，客户端断开连接），则中止另一个任务。
    // 确保当客户端断开连接时，服务器端能够及时中止这两个任务，释放资源
    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        // 这个分支等待 send_task 完成。_ 是一个占位符，表示我们不关心具体的值，只关心任务是否完成。
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    };

    // 当任务结束时，向所有订阅者发送 "user left" 消息，表示用户已离开。
    // Send "user left" message (similar to "joined" above).
    let msg = format!("{username} left.");
    tracing::debug!("{msg}");
    let _ = state.tx.send(msg);

    // 从用户集合中移除用户名，以便新客户端可以使用该用户名。
    // Remove username from map so new clients can take it again.
    state.user_set.lock().unwrap().remove(&username);
}

/// 这个函数检查用户名是否已被使用。如果未被使用，则将其添加到用户集合中，并将其保存到 string 变量中。
fn check_username(state: &AppState, string: &mut String, name: &str) {
    let mut user_set = state.user_set.lock().unwrap();

    if !user_set.contains(name) {
        user_set.insert(name.to_owned());

        string.push_str(name);
    }
}

// Include utf-8 file at **compile** time.
async fn index() -> Html<&'static str> {
    Html(std::include_str!("../../views/chat.html"))
}
