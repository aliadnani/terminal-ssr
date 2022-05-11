use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{routing::get, Extension, Router, Server};
use sysinfo::{System, SystemExt};

mod api;
mod display;
mod telemetry;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let sys = Arc::new(Mutex::new(System::new_all()));

    let app = Router::new()
        .route("/sse", get(api::server::graph_sse_handler))
        .layer(Extension(sys));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8910));

    tracing::debug!("Server started! Listening on: {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
