use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::{response::Html, routing::get, Router};

struct MyConfig {
    counter: AtomicUsize,
}

#[tokio::main]
async fn main() {
    let shared_config = Arc::new(MyConfig {
        counter: AtomicUsize::new(0),
    });

    // build our application with a single route
    let app = Router::new()
        .route("/", get(handler))
        .route("/book/:id", get(path_extract))
        .route("/book", get(query_extract))
        .route("/header", get(header_extract))
        .with_state(shared_config);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // run our app with hyper
    axum::serve(listener, app).await.unwrap();
}

// 路由处理函数
async fn handler(State(config): State<Arc<MyConfig>>) -> Html<String> {
    config
        .counter
        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Html(format!("<h1>Hello, Counter: {:?}!</h1>", config.counter))
}

// 添加路由参数提取功能
async fn path_extract(Path(id): Path<u32>) -> Html<String> {
    Html(format!("Book ID: {}", id))
}

async fn query_extract(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    Html(format!("{:?}", params))
}

async fn header_extract(headers: HeaderMap) -> Html<String> {
    Html(format!("{:?}", headers))
}
