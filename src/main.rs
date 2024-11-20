use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::{response::Html, routing::get, Router};

struct MyConfig {
    config_string: String,
}

#[tokio::main]
async fn main() {
    let shared_config = Arc::new(MyConfig {
        config_string: "My config value".to_string(),
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
    Html(format!("<h1>Hello, State: {:}!</h1>", config.config_string))
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
