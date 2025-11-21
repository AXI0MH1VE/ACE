pub mod dag;
pub mod model;
pub mod payment;
pub mod verification;

use crate::model::hybrid_block::HybridBlock;
use crate::verification::axiom_checker::{AxiomChecker, C0Signature};

/// Lightweight WASM entry-point for browser/edge usage.
pub fn wasm_generate(prompt: &str) -> (String, C0Signature) {
    let hybrid = HybridBlock::new(Default::default());
    let deterministic = hybrid.generate_verified(prompt);
    let checker = AxiomChecker::new();
    let sig = checker.verify_and_sign(&deterministic, "browser_default", 256);
    (deterministic, sig)
}
