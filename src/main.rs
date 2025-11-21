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

fn env_flag(key: &str, default: bool) -> bool {
    std::env::var(key)
        .ok()
        .map(|v| matches!(v.as_str(), "1" | "true" | "TRUE" | "yes" | "on"))
        .unwrap_or(default)
}

#[derive(Clone)]
struct SafetyPolicy {
    allow_network: bool,
    allow_verified: bool,
    require_payment: bool,
    high_risk_terms: Vec<String>,
    blocklist: Vec<String>,
}

impl Default for SafetyPolicy {
    fn default() -> Self {
        Self {
            allow_network: env_flag("AXIOMHIVE_ALLOW_NETWORK", false),
            allow_verified: true,
            require_payment: env_flag("AXIOMHIVE_REQUIRE_PAYMENT", true),
            high_risk_terms: vec![
                "financial advice".into(),
                "medical diagnosis".into(),
                "legal judgment".into(),
            ],
            blocklist: vec![
                "self-harm".into(),
                "bioweapon".into(),
                "malware".into(),
                "child exploitation".into(),
                "terrorism".into(),
            ],
        }
    }
}

#[derive(Debug)]
enum SafetyAction {
    Allow,
    Deny(String),
    Escalate(String),
}

impl SafetyPolicy {
    fn evaluate(
        &self,
        prompt: &str,
        mode: &str,
        axiom_set: Option<&str>,
        payment_token: Option<&str>,
        allow_network_request: bool,
        free_local: bool,
    ) -> SafetyAction {
        let lower = prompt.to_lowercase();
        if self
            .blocklist
            .iter()
            .any(|term| lower.contains(term))
        {
            return SafetyAction::Deny("Blocked content".into());
        }
        if mode == "verified" && !self.allow_verified {
            return SafetyAction::Deny("Verified mode disabled".into());
        }
        if mode == "verified" && self.require_payment && !free_local && payment_token.is_none() {
            return SafetyAction::Escalate("Payment required for verified mode".into());
        }
        let network_allowed = self.allow_network || allow_network_request;
        if !network_allowed {
            if let Some(ax) = axiom_set {
                if ax.contains("remote") {
                    return SafetyAction::Escalate("Network disabled; remote axiom set not allowed".into());
                }
            }
        }
        SafetyAction::Allow
    }
}

#[derive(Clone)]
struct AppState {
    hybrid: model::hybrid_block::HybridBlock,
    checker: verification::axiom_checker::AxiomChecker,
    billing: payment::bitcoin::LightningBilling,
    dag: dag::dag::DagScheduler,
    safety: SafetyPolicy,
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
    payment_token: Option<String>,
    allow_network: Option<bool>,
    free_local: Option<bool>,
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
        safety: SafetyPolicy::default(),
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
    match app
        .safety
        .evaluate(&body.prompt, "creative", None, None, false, false)
    {
        SafetyAction::Deny(reason) => {
            return Json(CreativeResponse {
                request_id: Uuid::new_v4(),
                output: format!("request denied: {reason}"),
                mode: "creative_denied".into(),
            })
        }
        SafetyAction::Escalate(reason) => {
            return Json(CreativeResponse {
                request_id: Uuid::new_v4(),
                output: format!("request requires escalation/consent: {reason}"),
                mode: "creative_pending".into(),
            })
        }
        SafetyAction::Allow => {}
    }

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
    match app
        .safety
        .evaluate(
            &body.prompt,
            "verified",
            body.axiom_set.as_deref(),
            body.payment_token.as_deref(),
            body.allow_network.unwrap_or(false),
            body.free_local.unwrap_or(false),
        )
    {
        SafetyAction::Deny(reason) => {
            return Json(VerifiedResponse {
                request_id: Uuid::new_v4(),
                output: format!("request denied: {reason}"),
                c0_signature: verification::axiom_checker::C0Signature::empty(),
                proof_uri: "".into(),
                merkle_root: "".into(),
            })
        }
        SafetyAction::Escalate(reason) => {
            return Json(VerifiedResponse {
                request_id: Uuid::new_v4(),
                output: format!("request requires escalation/consent: {reason}"),
                c0_signature: verification::axiom_checker::C0Signature::empty(),
                proof_uri: "".into(),
                merkle_root: "".into(),
            })
        }
        SafetyAction::Allow => {}
    }

    let request_id = Uuid::new_v4();
    let deterministic = app.hybrid.generate_verified(&body.prompt);
    let axiom_set = match body.axiom_set {
        Some(ax) => ax,
        None => {
            return Json(VerifiedResponse {
                request_id,
                output: "axiom_set is required for verified mode".into(),
                c0_signature: verification::axiom_checker::C0Signature::empty(),
                proof_uri: "".into(),
                merkle_root: "".into(),
            })
        }
    };
    let signature = app
        .checker
        .verify_and_sign(&deterministic, &axiom_set, body.max_steps.unwrap_or(1024));
    if signature.is_empty() {
        return Json(VerifiedResponse {
            request_id,
            output: "verification failed; no output emitted".into(),
            c0_signature: verification::axiom_checker::C0Signature::empty(),
            proof_uri: "".into(),
            merkle_root: "".into(),
        });
    }
    let merkle_root = app.dag.record_checkpoint(&deterministic);
    Json(VerifiedResponse {
        request_id,
        output: deterministic,
        c0_signature: signature,
        proof_uri: format!("zkml://proofs/{request_id}"),
        merkle_root,
    })
}
