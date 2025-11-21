use crate::{
    audit::AuditLog,
    dag::dag::{DagScheduler, RequestDag},
    model::engine::HybridModelEngine,
    payment::{bitcoin::LightningGateway, PaymentError},
    policy::{PolicyError, SafetyPolicy},
    verification::axiom_checker::{
        parse_axiom_set, C0Signature, DeterministicVerifier, VerificationError, Verifier,
    },
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub model: Arc<HybridModelEngine>,
    pub verifier: Arc<DeterministicVerifier<HybridModelEngine>>,
    pub policy: Arc<SafetyPolicy>,
    pub payment: Arc<LightningGateway>,
    pub dag: Arc<DagScheduler>,
    pub audit: Arc<AuditLog>,
    pub require_payment: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreativeRequest {
    pub prompt: String,
    pub media: Option<Vec<String>>,
    pub temperature: Option<f32>,
    pub top_k: Option<u32>,
    pub lightning_invoice: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreativeResponse {
    pub request_id: Uuid,
    pub output: String,
    pub mode: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifiedRequest {
    pub prompt: String,
    pub axiom_set: String,
    pub max_steps: Option<u32>,
    pub lightning_invoice: Option<String>,
    pub allow_network: Option<bool>,
    pub free_local: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct VerifiedResponse {
    pub request_id: Uuid,
    pub output: String,
    pub c0_signature: C0Signature,
    pub proof_uri: String,
    pub merkle_root: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub message: String,
}

impl ApiError {
    fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Json(ErrorResponse {
            error: self.message,
        });
        (self.status, body).into_response()
    }
}

fn env_flag(key: &str, default: bool) -> bool {
    std::env::var(key)
        .ok()
        .map(|v| matches!(v.as_str(), "1" | "true" | "TRUE" | "yes" | "on"))
        .unwrap_or(default)
}

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/api/v1/creative", post(handle_creative))
        .route("/api/v1/verified", post(handle_verified))
        .with_state(state)
}

pub fn build_state() -> anyhow::Result<AppState> {
    let policy = Arc::new(SafetyPolicy::load_from_disk("policy/safety_config.json")?);
    let require_payment = env_flag("AXIOMHIVE_REQUIRE_PAYMENT", true);

    let model = Arc::new(HybridModelEngine::default());
    let verifier = Arc::new(DeterministicVerifier::new(model.clone()));
    let payment = Arc::new(LightningGateway::new("axiomhive-edge"));
    let dag = Arc::new(DagScheduler::default());
    let audit_path =
        std::env::var("AXIOMHIVE_AUDIT_PATH").unwrap_or_else(|_| "data/audit.jsonl".into());
    let audit = Arc::new(AuditLog::new(audit_path));

    Ok(AppState {
        model,
        verifier,
        policy,
        payment,
        dag,
        audit,
        require_payment,
    })
}

pub async fn handle_creative(
    State(app): State<AppState>,
    Json(body): Json<CreativeRequest>,
) -> Result<Json<CreativeResponse>, ApiError> {
    if let Err(err) = app.policy.check_prompt(&body.prompt) {
        return Err(policy_error_to_api(err));
    }

    let temperature = body.temperature.unwrap_or(0.9);
    let top_k = body.top_k.unwrap_or(64);
    let media = body.media.unwrap_or_default();

    app.payment
        .validate_invoice(
            body.lightning_invoice.as_deref(),
            app.require_payment,
            false,
        )
        .await
        .map_err(payment_error_to_api)?;

    let output = app
        .model
        .generate_creative(&body.prompt, &media, temperature, top_k)
        .await
        .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let request_id = Uuid::new_v4();
    let mut dag = RequestDag::default();
    dag.add_node(
        "policy_check",
        json!({"prompt_len": body.prompt.len(), "mode": "creative"}),
    );
    dag.add_node(
        "model_run",
        json!({"temperature": temperature, "top_k": top_k, "media": media}),
    );

    let _merkle_root = app
        .dag
        .record_checkpoint(&dag)
        .unwrap_or_else(|| "empty-dag".to_string());
    app.audit
        .append(request_id, "creative", &dag)
        .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(CreativeResponse {
        request_id,
        output,
        mode: "creative".into(),
    }))
}

pub async fn handle_verified(
    State(app): State<AppState>,
    Json(body): Json<VerifiedRequest>,
) -> Result<Json<VerifiedResponse>, ApiError> {
    app.policy
        .check_prompt(&body.prompt)
        .map_err(policy_error_to_api)?;
    app.policy
        .ensure_verified_enabled()
        .map_err(policy_error_to_api)?;
    let allow_network = body
        .allow_network
        .unwrap_or_else(|| app.policy.allow_network_by_default());
    app.policy
        .ensure_network_allowed(allow_network)
        .map_err(policy_error_to_api)?;

    let free_local = body.free_local.unwrap_or(false);
    app.payment
        .validate_invoice(
            body.lightning_invoice.as_deref(),
            app.require_payment,
            free_local,
        )
        .await
        .map_err(payment_error_to_api)?;

    let axiom_set = parse_axiom_set(&body.axiom_set).map_err(verification_error_to_api)?;
    let max_steps = body.max_steps.unwrap_or(1024);

    let (output, c0_signature) = app
        .verifier
        .verify(&body.prompt, &axiom_set, max_steps)
        .await
        .map_err(verification_error_to_api)?;

    let request_id = Uuid::new_v4();
    let mut dag = RequestDag::default();
    dag.add_node(
        "policy_check",
        json!({"prompt_len": body.prompt.len(), "mode": "verified"}),
    );
    dag.add_node(
        "model_run",
        json!({"max_steps": max_steps, "axiom_set": axiom_set.name}),
    );
    dag.add_node(
        "verification",
        json!({"rules": axiom_set.rules.len(), "free_local": free_local}),
    );

    let _ = app.dag.record_checkpoint(&dag);
    let audit_entry = app
        .audit
        .append(request_id, "verified", &dag)
        .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(VerifiedResponse {
        request_id,
        output,
        c0_signature,
        proof_uri: format!("zkml://proofs/{request_id}"),
        merkle_root: audit_entry.merkle_root,
    }))
}

fn policy_error_to_api(err: PolicyError) -> ApiError {
    match err {
        PolicyError::Blocked(msg) => ApiError::new(StatusCode::FORBIDDEN, msg),
        PolicyError::VerifiedDisabled => {
            ApiError::new(StatusCode::FORBIDDEN, "verified mode disabled")
        }
        PolicyError::NetworkDisabled => {
            ApiError::new(StatusCode::FORBIDDEN, "network disabled for request")
        }
        PolicyError::EscalationRequired(msg) => ApiError::new(StatusCode::BAD_REQUEST, msg),
        PolicyError::ConfigLoad(msg) => ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, msg),
    }
}

fn payment_error_to_api(err: PaymentError) -> ApiError {
    match err {
        PaymentError::MissingOrInvalidInvoice => ApiError::new(
            StatusCode::PAYMENT_REQUIRED,
            "valid Lightning invoice required",
        ),
        PaymentError::PaymentNotRequired => ApiError::new(StatusCode::OK, "payment not required"),
    }
}

fn verification_error_to_api(err: VerificationError) -> ApiError {
    match err {
        VerificationError::InvalidAxiomSet(msg) => ApiError::new(StatusCode::BAD_REQUEST, msg),
        VerificationError::RuleFailed(rule) => ApiError::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            format!("rule failed: {rule}"),
        ),
        VerificationError::ModelFailure(msg) => {
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, msg)
        }
    }
}
