mod dag;
mod model;
mod payment;
mod verification;

use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::signal;
use tracing::info;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    hybrid: model::hybrid_block::HybridBlock,
    checker: verification::axiom_checker::AxiomChecker,
    billing: payment::bitcoin::LightningBilling,
    dag: dag::dag::DagScheduler,
}

#[derive(Debug, Deserialize)]
struct CreativeRequest {
    prompt: String,
    media: Option<Vec<String>>, // ["text", "image", "audio", "pdf"]
    temperature: Option<f32>,
    top_k: Option<usize>,
}

#[derive(Debug, Serialize)]
struct CreativeResponse {
    request_id: Uuid,
    output: String,
    mode: String,
}

#[derive(Debug, Deserialize)]
struct VerifiedRequest {
    prompt: String,
    axiom_set: Option<String>,
    max_steps: Option<u32>,
}

#[derive(Debug, Serialize)]
struct VerifiedResponse {
    request_id: Uuid,
    output: String,
    c0_signature: verification::axiom_checker::C0Signature,
    proof_uri: String,
    merkle_root: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let app_state = Arc::new(AppState {
        hybrid: model::hybrid_block::HybridBlock::new(model::hybrid_block::HybridBlockConfig::default()),
        checker: verification::axiom_checker::AxiomChecker::new(),
        billing: payment::bitcoin::LightningBilling::new("axiomhive-edge"),
        dag: dag::dag::DagScheduler::default(),
    });

    let api = Router::new()
        .route("/api/v1/creative", post(handle_creative))
        .route("/api/v1/verified", post(handle_verified))
        .with_state(app_state);

    let addr = "0.0.0.0:8090".parse()?;
    info!("Starting AxiomHive edge node on {addr}");
    axum::Server::bind(&addr)
        .serve(api.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let _ = signal::ctrl_c().await;
    info!("Shutdown signal received. Stopping AxiomHive node.");
}

async fn handle_creative(
    State(app): State<Arc<AppState>>,
    Json(body): Json<CreativeRequest>,
) -> Json<CreativeResponse> {
    let request_id = Uuid::new_v4();
    let output = app
        .hybrid
        .generate_creative(&body.prompt, body.temperature.unwrap_or(0.9), body.top_k.unwrap_or(64), body.media.unwrap_or_default());
    Json(CreativeResponse {
        request_id,
        output,
        mode: "creative".into(),
    })
}

async fn handle_verified(
    State(app): State<Arc<AppState>>,
    Json(body): Json<VerifiedRequest>,
) -> Json<VerifiedResponse> {
    let request_id = Uuid::new_v4();
    let deterministic = app.hybrid.generate_verified(&body.prompt);
    let axiom_set = body.axiom_set.unwrap_or_else(|| "default_finance_axioms".into());
    let signature = app
        .checker
        .verify_and_sign(&deterministic, &axiom_set, body.max_steps.unwrap_or(1024));
    let merkle_root = app.dag.record_checkpoint(&deterministic);
    Json(VerifiedResponse {
        request_id,
        output: deterministic,
        c0_signature: signature,
        proof_uri: format!("zkml://proofs/{request_id}"),
        merkle_root,
    })
}
