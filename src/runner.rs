// #![allow(missing_docs)]
use crate::http_client::HttpRequestConfig;
use serde::{Deserialize, Serialize};
use serde_json::{json,Value};
use serde_yaml;
use std::{
    future::IntoFuture,
    io::BufRead,
    process::{Command, Stdio},
    time::Duration
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpSocket},
    runtime::Builder,
    sync::oneshot,
};

//read file 

pub struct RunnerBuilder {
    pub name: &'static str,
    pub path: Option<&'static str>,
    pub method: Option<&'static str>,
    pub headers: Option<&'static [(&'static str, &'static str)]>,
    pub body: Option<Value>,
}


impl RunnerBuilder {

    pub async fn run(self) {

        let handles: Vec<_> = (0..3).map(|_| {
            tokio::spawn(async {
                for _ in 0..1 {
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
        
                    request_config.single_thread_send(3).await;
        
                }
            })

        }).collect();

        for handle in handles {
            handle.await.unwrap();
        };

    }
}

