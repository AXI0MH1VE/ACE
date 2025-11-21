use axiomhive::model::hybrid_block::{HybridBlock, HybridBlockConfig};
use axiomhive::verification::axiom_checker::AxiomChecker;

#[test]
fn creative_and_verified_paths_work() {
    let hybrid = HybridBlock::new(HybridBlockConfig::default());
    let creative = hybrid.generate_creative("hello", 0.7, 32, vec!["text".into()]);
    assert!(creative.contains("creative"));

    let verified = hybrid.generate_verified("hello");
    let checker = AxiomChecker::new();
    let sig = checker.verify_and_sign(&verified, "finance", 128);
    assert!(!sig.input_hash.is_empty());
    assert!(!sig.axiom_hash.is_empty());
    assert!(!sig.state_trace.is_empty());
    assert!(!sig.proof_cert.seal.is_empty());
}
