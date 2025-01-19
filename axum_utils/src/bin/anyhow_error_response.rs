//! cargo run -p axum_utils --bin anyhow-error-response

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // 创建一个新的路由器并定义路由规则
    let app = Router::new().route("/", get(handler));
    // 绑定到本地的 3000 端口，并启动一个异步 TCP 监听器
    let listener =
        tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap()); // Output: listening on 127.0.0.1:3000
    axum::serve(listener, app).await.unwrap();
}

/// handler 函数声明它返回 Result<(), AppError>，但是 try_thing 返回的是 anyhow::Error。为了让这段代码编译通过，
/// 你需要确保 AppError 可以从 anyhow::Error 转换而来。通常，这可以通过实现 From<anyhow::Error> for AppError 来完成。
/// anyhow::Error -> AppError
async fn handler() -> Result<(), AppError> {
    try_thing()?;
    Ok(())
}

fn try_thing() -> Result<(), anyhow::Error> {
    anyhow::bail!("it failed!")
}

// 包含一个 anyhow::Error 类型的字段。这样做的好处是可以利用 anyhow::Error 提供的丰富错误信息和上下文
// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// 将 AppError 转换为 HTTP 响应
// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // 当发生 AppError 时，服务器将返回一个 500 内部服务器错误状态码，以及一个包含错误信息的字符串。
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// anyhow::Error -> AppError
// 泛型参数 E: 为 AppError 实现了一个泛型的 From trait，其中 E 是一个泛型参数，代表任何类型。
// 约束 E: Into<anyhow::Error>: where E: Into<anyhow::Error> 是一个约束，表示类型 E 必须实现 Into<anyhow::Error> trait。这意味着 E 可以被转换为 anyhow::Error。
//
// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        // 将 err 转换为 anyhow::Error，然后用它来构造一个 AppError 实例
        Self(err.into())
    }
}
