use serde_json::{json};
mod http_client;
use http_client::HttpRequestConfig;
// use std::time::Duration;

use std::{thread::sleep, time::Duration};
use rand::Rng;
mod runner;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpSocket},
    runtime::Builder,
    sync::oneshot,
};

const TCP_ENDPOINT: &str = "127.0.0.1:8090";
const NUM_MSGS: usize = 100;
const MSG_SIZE: usize = 1024;

#[tokio::main]
async fn main() {
    /*
    name: &'static str,
    path: Option<&'static str>,
    method: Option<&'static str>,
    headers: Option<&'static [(&'static str, &'static str)]>,
    body: Option<&'static str>,
    */
    let name = "stres_test";
    let json_body = Some(json!({
                            "username": "test_user",
                            "password": "secure_password"
                        }));
    let method = Some("POST");
    let path = Some("http://127.0.0.1:8080/WebGoat/login");


    let run_builder = runner::RunnerBuilder{name: name,path: path,method: method,headers: None,body: json_body};
    run_builder.run().await;

    
    
    // let rt = Builder::new_multi_thread()
    // .enable_io()
    // .enable_time()
    // .build()
    // .unwrap();

    // rt.block_on(async {

    //     let handles: Vec<_> = (0..2).map(|_| {
    //         tokio::spawn(async {
    //             for _ in 0..2 {
    //                 // 构造 JSON body
    //                 let json_body = json!({
    //                     "username": "test_user",
    //                     "password": "secure_password"
    //                 });
        
    //                 // 发送 POST 请求
    //                 let mut request_config = HttpRequestConfig::new(
    //                     "POST",
    //                     "http://127.0.0.1:8080/WebGoat/login",
    //                     None,
    //                     Some(json_body),
    //                     None,
    //                     Some(Duration::from_secs(10)), // 设置超时时间
    //                     Some(true),  // 启用 Cookie
    //                     None,
    //                 ).expect("Failed to create request config");
        
    //                 request_config.single_thread_send(3).await;
        
    //             }
    //         })

    //     }).collect();

    //     for handle in handles {
    //         handle.await.unwrap();
    //     }

    // });
    
    

    // request_config.single_thread_send(3).await;

    
    // let rt2 = Builder::new_multi_thread().enable_io().build().unwrap();

    // rt.spawn(async {
    //     let listener = TcpListener::bind(TCP_ENDPOINT).await.unwrap();
    //     let (mut socket, _) = listener.accept().await.unwrap();
    //     let (mut rd, mut wr) = socket.split();
    //     while tokio::io::copy(&mut rd, &mut wr).await.is_ok() {}
    // });

    // // wait a bit so that the listener binds.
    // sleep(Duration::from_millis(100));

    // // create a channel to let the main thread know that all the messages were sent and received.
    // let (tx, mut rx) = oneshot::channel();

    // rt2.spawn(async {
    //     let addr = TCP_ENDPOINT.parse().unwrap();
    //     let socket = TcpSocket::new_v4().unwrap();
    //     let mut stream = socket.connect(addr).await.unwrap();

    //     let mut buff = [0; MSG_SIZE];
    //     for _ in 0..NUM_MSGS {
    //         let one_mega_random_bytes: Vec<u8> =
    //             (0..MSG_SIZE).map(|_| rand::random::<u8>()).collect();
    //         stream
    //             .write_all(one_mega_random_bytes.as_slice())
    //             .await
    //             .unwrap();
    //         let _ = stream.read(&mut buff).await.unwrap();
    //     }
    //     tx.send(()).unwrap();
    // });

    // loop {
    //     // check that we're done.
    //     match rx.try_recv() {
    //         Err(oneshot::error::TryRecvError::Empty) => (),
    //         Err(oneshot::error::TryRecvError::Closed) => panic!("channel got closed..."),
    //         Ok(()) => break,
    //     }
    // }

}
