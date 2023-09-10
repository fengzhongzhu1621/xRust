#![deny(warnings)]

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

type HttpClient = Client<hyper::client::HttpConnector>;

#[tokio::main]
async fn main() {
    // 创建 ip v4套接字地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 8100));

    // 创建一个HTTP客户端，将使用该客户端将请求转发到目标URL。
    let client = Client::builder()
        .http1_title_case_headers(true)
        .http1_preserve_header_case(true)
        .build_http();

    // 定义转发服务
    // 创建一个处理传入请求的新服务，并将它们传递给代理函数进行处理。
    // 使用Rust的async/await语法定义了一个异步函数，该函数返回一个Result，其中包含一个有效的服务或一个错误。
    let make_service = make_service_fn(move |_| {
        let client = client.clone();
        async move { Ok::<_, Infallible>(service_fn(move |req| proxy(client.clone(), req))) }
    });

    // 创建一个新的HTTP服务器，该服务器侦听指定的套接字地址
    // 启动服务器并使用指定的服务为传入的请求提供服务。
    let server = Server::bind(&addr)
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve(make_service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn proxy(_client: HttpClient, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // 克隆传入的请求头并将它们打印到控制台
    let headers = req.headers().clone();
    println!("headers: {:?}", headers);

    // 提取请求路径并检查它是否以“/hello”开头
    let path = req.uri().path().to_string();
    if path.starts_with("/hello") {
        let target_url = "http://127.0.0.1:8000".to_owned();
        // 创建一个新的HTTP请求到目标URL，并将响应返回给客户端
        let resp = get_response(_client, req, &target_url, &path).await?;
        return Ok(resp);
    }

    let resp = Response::new(Body::from("sorry! no route found"));
    Ok(resp)
}

/// 创建一个新的HTTP请求到目标URL
async fn get_response(
    client: HttpClient,
    req: Request<Body>,
    target_url: &str,
    path: &str,
) -> Result<Response<Body>, hyper::Error> {
    // 通过连接目标URL和传入请求路径来构造完整的目标URL
    let target_url = format!("{}{}", target_url, path);
    // 克隆传入的请求头，并使用它们构造一个新的HTTP请求，该请求具有与传入请求相同的方法、URI和主体。
    let headers = req.headers().clone();
    let mut request_builder = Request::builder()
        .method(req.method())
        .uri(target_url)
        .body(req.into_body())
        .unwrap();

    *request_builder.headers_mut() = headers;
    // 发送请求，然后等待响应并将其返回给调用者
    let response = client.request(request_builder).await?;
    let body = hyper::body::to_bytes(response.into_body()).await?;
    let body = String::from_utf8(body.to_vec()).unwrap();

    let mut resp = Response::new(Body::from(body));
    *resp.status_mut() = http::StatusCode::OK;
    Ok(resp)
}
