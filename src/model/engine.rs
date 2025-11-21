use super::hybrid_block::{HybridBlock, HybridBlockConfig};
use async_trait::async_trait;
use blake3::Hasher;
use serde_json::Value;

#[async_trait]
pub trait ModelEngine: Send + Sync {
    async fn generate_creative(
        &self,
        prompt: &str,
        media: &[String],
        temperature: f32,
        top_k: u32,
    ) -> anyhow::Result<String>;

    async fn generate_verified(
        &self,
        prompt: &str,
        axiom_set: &Value,
        max_steps: u32,
    ) -> anyhow::Result<String>;
}

#[derive(Clone)]
pub struct HybridModelEngine {
    block: HybridBlock,
}

impl Default for HybridModelEngine {
    fn default() -> Self {
        Self {
            block: HybridBlock::new(HybridBlockConfig::default()),
        }
    }
}

impl HybridModelEngine {
    pub fn new(config: HybridBlockConfig) -> Self {
        Self {
            block: HybridBlock::new(config),
        }
    }

    fn seed_from_inputs(parts: &[&str]) -> [u8; 32] {
        let mut hasher = Hasher::new();
        for part in parts {
            hasher.update(part.as_bytes());
        }
        let digest = hasher.finalize();
        *digest.as_bytes()
    }
}

#[async_trait]
impl ModelEngine for HybridModelEngine {
    async fn generate_creative(
        &self,
        prompt: &str,
        media: &[String],
        temperature: f32,
        top_k: u32,
    ) -> anyhow::Result<String> {
        let media_join = media.join("|");
        let seed = Self::seed_from_inputs(&[
            prompt,
            &media_join,
            &temperature.to_string(),
            &top_k.to_string(),
        ]);
        let output = self
            .block
            .generate_creative(prompt, media, temperature, top_k as usize, seed);
        Ok(output)
    }

    async fn generate_verified(
        &self,
        prompt: &str,
        axiom_set: &Value,
        max_steps: u32,
    ) -> anyhow::Result<String> {
        let axiom_fingerprint = blake3::hash(serde_json::to_string(axiom_set)?.as_bytes())
            .to_hex()
            .to_string();
        let seed = Self::seed_from_inputs(&[prompt, &axiom_fingerprint, &max_steps.to_string()]);
        // include derived seed in output to ensure deterministic but distinct responses across inputs
        let deterministic = self
            .block
            .generate_verified(prompt, &axiom_fingerprint, max_steps);
        let composed = format!("{deterministic} seed={}", hex::encode(seed));
        Ok(composed)
    }
}
