mod routes;
mod types;

use crate::routes::get_router;
use listenfd::ListenFd;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                concat!(env!("CARGO_PKG_NAME"), "=debug,tower_http=debug").into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        None => {
            let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
            TcpListener::bind(addr).await.unwrap()
        }
    };
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    let serve_dir =
        ServeDir::new("client/dist").not_found_service(ServeFile::new("client/dist/index.html"));
    let app = get_router().fallback_service(serve_dir);
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}
