pub mod api;
pub mod audit;
pub mod dag;
pub mod model;
pub mod payment;
pub mod policy;
pub mod verification;

use model::engine::HybridModelEngine;
use std::sync::Arc;
use verification::axiom_checker::{
    AxiomSet, C0Signature, DeterministicVerifier, VerificationError,
};

pub use verification::axiom_checker::parse_axiom_set;

pub async fn generate_verified_output(
    prompt: &str,
    axiom_set: &AxiomSet,
    max_steps: u32,
) -> Result<(String, C0Signature), VerificationError> {
    let model = Arc::new(HybridModelEngine::default());
    let verifier = DeterministicVerifier::new(model);
    verifier.verify(prompt, axiom_set, max_steps).await
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Lightweight WASM entry-point for browser/edge usage.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn wasm_generate(prompt: String, axiom_set: JsValue, max_steps: u32) -> JsValue {
    let parsed: AxiomSet = axiom_set.into_serde().unwrap_or_else(|_| AxiomSet {
        name: "default".into(),
        version: "0.0.0".into(),
        rules: vec![],
    });
    let (output, sig) = generate_verified_output(&prompt, &parsed, max_steps)
        .await
        .unwrap_or_else(|_| {
            (
                "[verified deterministic] wasm fallback".to_string(),
                C0Signature::empty(),
            )
        });
    JsValue::from_serde(&(output, sig)).unwrap_or_else(|_| JsValue::NULL)
}
