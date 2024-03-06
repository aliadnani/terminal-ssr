use std::{sync::Arc, time::Duration};

use axum::{routing::get, Router};
use sysinfo::System;
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

use crate::{config::config::Config, telemetry::sys::CachingSystemInfoService};

mod api;
mod config;
mod display;
mod telemetry;

#[tokio::main]
async fn main() {
    // Load Config
    let app_config = Config::load_config_from_env_with_defaults();

    // Logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&app_config.rust_log))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting 'terminal-ssr' with config: {:?}", &app_config);

    // System info
    tracing::info!("Initializing system info service.");
    let system = Arc::new(Mutex::new(System::new_all()));
    let system_info_service = Arc::new(Mutex::new(CachingSystemInfoService::new(system).await));

    tracing::info!("Starting system info scheduled background refresh process.");
    let _ = CachingSystemInfoService::schedule_refresh(
        system_info_service.clone(),
        Duration::from_millis(500),
    );

    // Server
    let app = Router::new()
        .route("/info", get(api::server::graph_sse_handler))
        .with_state(system_info_service)
        .layer(TraceLayer::new_for_http());

    let host = &app_config.server_host;

    let listener = tokio::net::TcpListener::bind(host).await.unwrap();

    tracing::info!("Server started! Listening on: {}", host);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
