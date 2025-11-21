use axiomhive::api::{build_router, build_state, AppState};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;

fn setup_state() -> AppState {
    let tmp = tempfile::tempdir().expect("tmpdir");
    std::env::set_var("AXIOMHIVE_AUDIT_PATH", tmp.path().join("audit.jsonl"));
    std::env::set_var("AXIOMHIVE_REQUIRE_PAYMENT", "0");
    build_state().expect("build state")
}

#[tokio::test]
async fn creative_contract_returns_expected_shape() {
    let state = setup_state();
    let app = build_router(state);

    let payload = json!({
        "prompt": "hello creative",
        "media": ["text"],
        "temperature": 0.7,
        "top_k": 16
    });

    let response = app
        .oneshot(
            Request::post("/api/v1/creative")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json.get("request_id").is_some());
    assert!(json.get("output").is_some());
    assert_eq!(json.get("mode").unwrap(), "creative");
}

#[tokio::test]
async fn verified_contract_returns_signature() {
    let state = setup_state();
    let app = build_router(state);

    let payload = json!({
        "prompt": "deterministic hello",
        "axiom_set": r#"{"name":"demo","version":"1","rules":[{"id":"contains-hello","must_contain":["hello"]}]}"#,
        "max_steps": 32,
        "allow_network": false,
        "free_local": true
    });

    let response = app
        .oneshot(
            Request::post("/api/v1/verified")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json.get("request_id").is_some());
    assert!(json.get("output").is_some());
    let sig = json.get("c0_signature").unwrap();
    assert!(sig.get("input_hash").is_some());
    assert!(sig.get("axiom_hash").is_some());
    assert!(sig.get("proof_cert").is_some());
}
