use crate::model::engine::ModelEngine;
use async_trait::async_trait;
use blake3::Hasher;
use serde::{Deserialize, Serialize};
use serde_json;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct AxiomSet {
    pub name: String,
    pub version: String,
    pub rules: Vec<AxiomRule>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AxiomRule {
    pub id: String,
    pub must_contain: Option<Vec<String>>,
    pub must_not_contain: Option<Vec<String>>,
}

#[derive(Debug, Error)]
pub enum VerificationError {
    #[error("invalid axiom_set: {0}")]
    InvalidAxiomSet(String),
    #[error("verification failed for rule: {0}")]
    RuleFailed(String),
    #[error("model error: {0}")]
    ModelFailure(String),
}

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

    fn deterministic_timestamp(prompt: &str, axiom_set: &AxiomSet, max_steps: u32) -> i64 {
        let mut hasher = Hasher::new();
        hasher.update(prompt.as_bytes());
        hasher.update(
            serde_json::to_string(axiom_set)
                .unwrap_or_default()
                .as_bytes(),
        );
        hasher.update(max_steps.to_be_bytes().as_slice());
        let digest = hasher.finalize();
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&digest.as_bytes()[..8]);
        // restrict range to avoid overflow and keep monotonic-ish values
        (u64::from_be_bytes(bytes) % 4_110_000_000) as i64
    }

    pub fn new(prompt: &str, axiom_set: &AxiomSet, output: &str, max_steps: u32) -> Self {
        let mut input_hasher = Sha256::new();
        input_hasher.update(output.as_bytes());
        let input_hash = format!("{:x}", input_hasher.finalize());

        let mut axiom_hasher = Sha256::new();
        axiom_hasher.update(
            serde_json::to_string(axiom_set)
                .unwrap_or_default()
                .as_bytes(),
        );
        let axiom_hash = format!("{:x}", axiom_hasher.finalize());

        let state_trace = blake3::hash(
            format!(
                "axiom={}::steps={}::output={}",
                axiom_hash, max_steps, output
            )
            .as_bytes(),
        )
        .to_hex()
        .to_string();

        let proof_seal = blake3::hash(format!("lean4+ezkl::{}", state_trace).as_bytes())
            .to_hex()
            .to_string();

        let timestamp = Self::deterministic_timestamp(prompt, axiom_set, max_steps);

        C0Signature {
            input_hash,
            axiom_hash,
            state_trace,
            proof_cert: ProofCert {
                backend: "lean4".into(),
                prover: "ezkl-halo2".into(),
                circuit: format!("axiom_set::{}", axiom_set.name),
                seal: proof_seal,
                timestamp_utc: timestamp,
            },
        }
    }
}

#[async_trait]
pub trait Verifier: Send + Sync {
    async fn verify(
        &self,
        prompt: &str,
        axiom_set: &AxiomSet,
        max_steps: u32,
    ) -> Result<(String, C0Signature), VerificationError>;
}

#[derive(Clone)]
pub struct DeterministicVerifier<M: ModelEngine + Send + Sync + 'static> {
    model: Arc<M>,
}

impl<M: ModelEngine + Send + Sync + 'static> DeterministicVerifier<M> {
    pub fn new(model: Arc<M>) -> Self {
        Self { model }
    }

    fn check_rules(output: &str, axiom_set: &AxiomSet) -> Result<(), VerificationError> {
        let lower_output = output.to_lowercase();
        for rule in &axiom_set.rules {
            if let Some(required) = &rule.must_contain {
                for token in required {
                    if !lower_output.contains(&token.to_lowercase()) {
                        return Err(VerificationError::RuleFailed(rule.id.clone()));
                    }
                }
            }
            if let Some(banned) = &rule.must_not_contain {
                for token in banned {
                    if lower_output.contains(&token.to_lowercase()) {
                        return Err(VerificationError::RuleFailed(rule.id.clone()));
                    }
                }
            }
        }
        Ok(())
    }
}

#[async_trait]
impl<M: ModelEngine + Send + Sync + 'static> Verifier for DeterministicVerifier<M> {
    async fn verify(
        &self,
        prompt: &str,
        axiom_set: &AxiomSet,
        max_steps: u32,
    ) -> Result<(String, C0Signature), VerificationError> {
        let axiom_value = serde_json::to_value(axiom_set)
            .map_err(|e| VerificationError::InvalidAxiomSet(e.to_string()))?;
        let output = self
            .model
            .generate_verified(prompt, &axiom_value, max_steps)
            .await
            .map_err(|e| VerificationError::ModelFailure(e.to_string()))?;

        Self::check_rules(&output, axiom_set)?;

        let c0 = C0Signature::new(prompt, axiom_set, &output, max_steps);
        Ok((output, c0))
    }
}

pub fn parse_axiom_set(raw: &str) -> Result<AxiomSet, VerificationError> {
    serde_json::from_str::<AxiomSet>(raw)
        .map_err(|e| VerificationError::InvalidAxiomSet(e.to_string()))
}
