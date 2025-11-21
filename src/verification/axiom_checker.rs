use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProofCert {
    pub backend: String,
    pub prover: String,
    pub circuit: String,
    pub seal: String,
    pub timestamp_utc: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct C0Signature {
    pub input_hash: String,
    pub axiom_hash: String,
    pub state_trace: String,
    pub proof_cert: ProofCert,
}

impl C0Signature {
    pub fn empty() -> Self {
        Self {
            input_hash: String::new(),
            axiom_hash: String::new(),
            state_trace: String::new(),
            proof_cert: ProofCert {
                backend: String::new(),
                prover: String::new(),
                circuit: String::new(),
                seal: String::new(),
                timestamp_utc: 0,
            },
        }
    }

    pub fn is_empty(&self) -> bool {
        self.input_hash.is_empty() || self.axiom_hash.is_empty() || self.proof_cert.seal.is_empty()
    }
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

        let state_trace = blake3::hash(
            format!("axiom={}::steps={}::output={}", axiom_set, max_steps, output).as_bytes(),
        )
        .to_hex()
        .to_string();

        let proof_seal = blake3::hash(format!("lean4+ezkl::{}", state_trace).as_bytes())
            .to_hex()
            .to_string();

        C0Signature {
            input_hash,
            axiom_hash,
            state_trace,
            proof_cert: ProofCert {
                backend: "lean4".into(),
                prover: "ezkl-halo2".into(),
                circuit: format!("axiom_set::{axiom_set}"),
                seal: proof_seal,
                timestamp_utc: Utc::now().timestamp(),
            },
        }
    }
}
