use httpmock::prelude::*;
use isahc;

#[test]
fn test_start() {
    // 启动轻量级模拟服务器
    let server = MockServer::start();

    // 创建一个 mock
    let mock = server.mock(|when, then| {
        when.method(GET).path("/translate").query_param("word", "hello");
        then.status(200)
            .header("content-type", "text/html; charset=UTF-8")
            .body("Привет");
    });

    // 向 mock 服务器发送请求
    let response = isahc::get(server.url("/translate?word=hello")).unwrap();
    mock.assert();

    // 验证响应
    assert_eq!(response.status(), 200);
}
