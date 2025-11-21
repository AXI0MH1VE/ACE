use axiomhive::model::engine::HybridModelEngine;
use axiomhive::verification::axiom_checker::{parse_axiom_set, DeterministicVerifier};
use futures::executor::block_on;
use std::sync::Arc;

#[test]
fn verified_request_succeeds_when_rules_met() {
    let verifier = DeterministicVerifier::new(Arc::new(HybridModelEngine::default()));
    let axiom_set = r#"{
        "name": "demo",
        "version": "1",
        "rules": [{"id":"contains-hello","must_contain":["verified"]}]
    }"#;
    let parsed = parse_axiom_set(axiom_set).unwrap();
    let result = block_on(verifier.verify("hello", &parsed, 32)).unwrap();
    assert!(result.0.contains("verified"));
}

#[test]
fn missing_required_token_fails() {
    let verifier = DeterministicVerifier::new(Arc::new(HybridModelEngine::default()));
    let axiom_set = r#"{
        "name": "demo",
        "version": "1",
        "rules": [{"id":"needs-token","must_contain":["nonexistent"]}]
    }"#;
    let parsed = parse_axiom_set(axiom_set).unwrap();
    let err = block_on(verifier.verify("hello", &parsed, 32)).unwrap_err();
    assert!(format!("{err:?}").contains("needs-token"));
}

#[test]
fn banned_token_triggers_failure() {
    let verifier = DeterministicVerifier::new(Arc::new(HybridModelEngine::default()));
    let axiom_set = r#"{
        "name": "demo",
        "version": "1",
        "rules": [{"id":"ban-verified","must_not_contain":["verified deterministic"]}]
    }"#;
    let parsed = parse_axiom_set(axiom_set).unwrap();
    let err = block_on(verifier.verify("hello", &parsed, 32)).unwrap_err();
    assert!(format!("{err:?}").contains("ban-verified"));
}
