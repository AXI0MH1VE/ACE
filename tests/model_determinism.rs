use axiomhive::model::engine::HybridModelEngine;
use futures::executor::block_on;
use serde_json::json;

#[test]
fn verified_generation_is_deterministic() {
    let engine = HybridModelEngine::default();
    let axiom_set = json!({"name":"demo","version":"1","rules":[]});
    let a = block_on(engine.generate_verified("repeat", &axiom_set, 42)).unwrap();
    let b = block_on(engine.generate_verified("repeat", &axiom_set, 42)).unwrap();
    assert_eq!(a, b);
}

#[test]
fn creative_generation_depends_on_inputs() {
    let engine = HybridModelEngine::default();
    let media = vec!["text".to_string()];
    let sample_a = block_on(engine.generate_creative("prompt", &media, 0.5, 16)).unwrap();
    let sample_b = block_on(engine.generate_creative("prompt", &media, 0.5, 16)).unwrap();
    assert_eq!(sample_a, sample_b);

    let sample_c = block_on(engine.generate_creative("prompt", &media, 0.7, 16)).unwrap();
    assert_ne!(sample_a, sample_c);
}
