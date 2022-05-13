use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};

use axum::{routing::get, Extension, Router, Server};
use sysinfo::{System, SystemExt};
use tokio::time::sleep;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

mod api;
mod display;
mod telemetry;

#[tokio::main]
async fn main() {
    // Logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // System info
    let mut sys = System::new_all();

    tracing::info!("Starting system info worker");
    for _ in 0..5 {
        // Refresh several times as documentation says first readings won't be accurate
        sys.refresh_all();
        sleep(Duration::from_millis(200)).await;
    }

    let sys = Arc::new(Mutex::new(sys));

    // Server
    let app = Router::new()
        .route("/info", get(api::server::graph_sse_handler))
        .layer(Extension(sys))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8910));

    tracing::info!("Server started! Listening on: {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
