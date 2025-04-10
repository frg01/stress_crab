
## Requirment, install OpenSSL
```shell
#linux - unbuntu
sudo apt update
#1. insall pkg-config,Rust will be use it. 
sudo apt install pkg-config
#2. install libssl-dev,reqwest will be need it on linux.
sudo apt install libssl-dev
#3. export environment 
export OPENSSL_DIR=/usr
export OPENSSL_INCLUDE_DIR=/usr/include
export OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu
export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig
```


## interface types of test aim
1. RESTful API: reqwest。
2. RPC、gRPC、JSON-RPC:: jsonrpc(JSON-RPC),tonic(gRPC)。
3. WebSocket: tokio-tungstenite(async WebSocket)。
4. GraphQL: async-graphql

## testing framework 
- rstest: Fixture and paramterize test
- cargo-test-junit: generate JUnit format test report.(cargo test-junit)
- tracing + tracing-appender: async write log.