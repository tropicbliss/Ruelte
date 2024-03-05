use axum::{response::IntoResponse, routing::get, Router};

pub fn get_router() -> Router {
    Router::new().route("/hello", get(hello))
}

async fn hello() -> impl IntoResponse {
    "Hi from /hello"
}
