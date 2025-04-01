use serde_json::{json};
mod http_client;
use http_client::HttpRequestConfig;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let body = reqwest::get("http://www.baidu.com")
        .await?
        .text()
        .await?;
    
    //println!("body= {body:?}");
    
    let params = [("foo","bar"),("baz","quux")];
    let client = reqwest::Client::new();
    let res = client.post("http://www.baidu.com")
        .form(&params)
        .send()
        .await?;
    //println!("res={res:?}");
    
    // 构造 JSON body
    let json_body = json!({
        "username": "test_user",
        "password": "secure_password"
    });

    // 发送 POST 请求
    let mut request_config = HttpRequestConfig::new(
        "POST",
        "http://127.0.0.1:8080/WebGoat/login",
        None,
        Some(json_body),
        None,
        Some(Duration::from_secs(10)), // 设置超时时间
        Some(true),  // 启用 Cookie
        None,
    ).expect("Failed to create request config");

    // 初始化 Client
    //request_config = request_config.init_client().await.expect("Failed to initialize client");

    // 发送请求
    match request_config.send().await {
        Ok(response) => {
            println!("Status: {}", response.status());
            println!("Headers: {:#?}", response.headers());
            let body = response.text().await?;
            println!("Response: {}", body);
        }
        Err(e) => {
            println!("Request failed: {}", e);
        }
    }

    match request_config.send().await {
        Ok(response) => {
            println!("Status: {}", response.status());
            println!("Headers: {:#?}", response.headers());
            let body = response.text().await?;
            println!("Response: {}", body);
        }
        Err(e) => {
            println!("Request failed: {}", e);
        }
    }


    Ok(())

}
