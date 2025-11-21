use axiomhive::model::engine::HybridModelEngine;
use axiomhive::model::hybrid_block::{HybridBlock, HybridBlockConfig};
use axiomhive::verification::axiom_checker::{parse_axiom_set, DeterministicVerifier, Verifier};
use std::sync::Arc;

#[test]
fn creative_and_verified_paths_work() {
    let hybrid = HybridBlock::new(HybridBlockConfig::default());
    let creative = hybrid.generate_creative_default("hello", 0.7, 32, vec!["text".into()]);
    assert!(creative.contains("creative"));

    let axiom_set = r#"{
        "name": "finance",
        "version": "1.0",
        "rules": [{
            "id": "must-include-hello",
            "must_contain": ["hello"],
            "must_not_contain": ["forbidden"]
        }]
    }"#;
    let parsed = parse_axiom_set(axiom_set).expect("axiom parse");
    let engine = Arc::new(HybridModelEngine::default());
    let verifier = DeterministicVerifier::new(engine);
    let (output, sig) =
        futures::executor::block_on(verifier.verify("hello", &parsed, 128)).expect("verified");
    assert!(output.contains("verified"));
    assert!(!sig.input_hash.is_empty());
    assert!(!sig.axiom_hash.is_empty());
    assert!(!sig.state_trace.is_empty());
    assert!(!sig.proof_cert.seal.is_empty());
}
