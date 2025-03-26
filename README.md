sudo apt update
//检测检测库路径的工具，Rust 依赖它来找到 OpenSSL。
sudo apt install pkg-config
//可能缺少 OpenSSL,reqwest需要用到linux的这个库
sudo apt install libssl-dev
//手动指定 OpenSSL 位置
export OPENSSL_DIR=/usr
export OPENSSL_INCLUDE_DIR=/usr/include
export OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu
export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig


## 测试进行的接口类型
1. RESTful API: 使用reqwest库。
2. RPC、gRPC、JSON-RPC:: 使用jsonrpc(模拟JSON-RPC 请求并验证响应),tonic(适用于gRPC的库，支持异步和同步操作)。
3. WebSocket: tokio-tungstenite(用于异步WebSocket通信)。
4. GraphQL: async-graphql 提供GraphQL服务的实现，也可以用于客户端查询和测试。
