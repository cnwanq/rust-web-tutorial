use std::collections::HashMap;

use axum::extract::{Path, Query};
use axum::http::HeaderMap;
use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(handler))
        .route("/book/:id", get(path_extract))
        .route("/book", get(query_extract))
        .route("/header", get(header_extract));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // run our app with hyper
    axum::serve(listener, app).await.unwrap();
}

// 路由处理函数
async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
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
