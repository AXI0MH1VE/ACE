mod api;
mod audit;
mod dag;
mod model;
mod payment;
mod policy;
mod verification;

use api::{build_router, build_state};
use std::net::SocketAddr;
use tokio::{net::TcpListener, signal};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let state = build_state()?;
    let app = build_router(state);

    let addr: SocketAddr = "127.0.0.1:8090".parse()?;
    info!("Starting AxiomHive edge node on http://{addr}");

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let _ = signal::ctrl_c().await;
    info!("Shutdown signal received. Stopping AxiomHive node.");
}
