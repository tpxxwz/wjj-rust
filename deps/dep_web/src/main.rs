use axum::{Router, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    axum_hello_world().await
}

async fn axum_hello_world() {
    // 定义路由
    let app = Router::new().route("/", get(root));

    // 创建 TcpListener
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!(
        "Server running at http://{}",
        listener.local_addr().unwrap()
    );

    // 启动服务
    axum::serve(listener, app).await.unwrap();
}

// 处理函数
async fn root() -> &'static str {
    "Hello, Axum!"
}
