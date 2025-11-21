use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct C0Signature {
    pub input_hash: String,
    pub axiom_hash: String,
    pub state_trace: String,
    pub proof_cert: String,
}

#[derive(Clone, Default)]
pub struct AxiomChecker;

impl AxiomChecker {
    pub fn new() -> Self {
        Self
    }

    pub fn verify_and_sign(&self, output: &str, axiom_set: &str, max_steps: u32) -> C0Signature {
        let mut input_hasher = Sha256::new();
        input_hasher.update(output.as_bytes());
        let input_hash = format!("{:x}", input_hasher.finalize());

        let mut axiom_hasher = Sha256::new();
        axiom_hasher.update(axiom_set.as_bytes());
        let axiom_hash = format!("{:x}", axiom_hasher.finalize());

        C0Signature {
            input_hash,
            axiom_hash,
            state_trace: format!("state_trace:steps={max_steps}"),
            proof_cert: "lean4:placeholder-proof-cert".into(),
        }
    }
}
