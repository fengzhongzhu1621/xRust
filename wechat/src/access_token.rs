use anyhow::{bail, Result};
use reqwest;
use serde_json::Value;

/// 获取微信公众号的access_token
async fn fetch_access_token(app_id: &str, app_secret: &str) -> Result<String> {
    let url = format!(
        "https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={}&secret={}",
        app_id, app_secret
    );
    // 异步请求获取acess_token
    let resp = reqwest::get(url).await?;
    let body = resp.text().await?;
    let json_data: Value = serde_json::from_str(&body)?;
    let access_token = &json_data["access_token"];

    if access_token == &Value::Null {
        let errmsg = format!(
            "get access token error,error info is {{code:{} msg:{}}}",
            json_data["errcode"], json_data["errmsg"]
        );
        // 返回Error
        bail!(errmsg)
    }

    // 转换为String格式
    let token = access_token.as_str().unwrap();
    Ok(token.to_string())
}
