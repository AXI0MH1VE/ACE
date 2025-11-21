use rand::Rng;

use super::attention::AttentionHead;
use super::meta_tokens::MetaTokenInjector;
use super::ssm::SsmHead;

#[derive(Clone, Debug)]
pub struct HybridBlockConfig {
    pub ssm_heads: usize,
    pub attention_heads: usize,
    pub meta_tokens: usize,
    pub kv_stride: usize,
    pub sliding_window: usize,
}

impl Default for HybridBlockConfig {
    fn default() -> Self {
        Self {
            ssm_heads: 8,          // 85% SSM in a 5:1 ratio
            attention_heads: 2,    // 15% attention
            meta_tokens: 8,        // creative default (use 16 in verified mode)
            kv_stride: 2,
            sliding_window: 2048,
        }
    }
}

#[derive(Clone)]
pub struct HybridBlock {
    pub config: HybridBlockConfig,
    ssm_heads: Vec<SsmHead>,
    attention_heads: Vec<AttentionHead>,
    injector: MetaTokenInjector,
}

impl HybridBlock {
    pub fn new(config: HybridBlockConfig) -> Self {
        let ssm_heads = (0..config.ssm_heads).map(|_| SsmHead::new()).collect();
        let attention_heads = (0..config.attention_heads)
            .map(|_| AttentionHead::new(config.sliding_window))
            .collect();
        let injector = MetaTokenInjector::new(config.meta_tokens);
        Self {
            config,
            ssm_heads,
            attention_heads,
            injector,
        }
    }

    pub fn generate_creative(&self, prompt: &str, temperature: f32, top_k: usize, media: Vec<String>) -> String {
        let mut rng = rand::thread_rng();
        let injected = self.injector.inject(prompt, "creative");
        let ssm_trace = self
            .ssm_heads
            .iter()
            .map(|h| h.step(&injected))
            .collect::<Vec<_>>()
            .join(" | ");
        let attn_trace = self
            .attention_heads
            .iter()
            .map(|h| h.attend(&injected))
            .collect::<Vec<_>>()
            .join(" | ");
        let mut sample = format!(
            "[creative fused] meta={} ssm=[{}] attn=[{}] temp={temperature} top_k={top_k}",
            self.config.meta_tokens, ssm_trace, attn_trace
        );
        if !media.is_empty() {
            sample.push_str(&format!(" media={:?}", media));
        }
        // lightweight randomness to mimic sampling behavior without a real model
        let noise: u32 = rng.gen_range(0..9999);
        format!("{sample} :: sample_id={noise}")
    }

    pub fn generate_verified(&self, prompt: &str) -> String {
        let injected = self.injector.inject(prompt, "verified");
        let ssm = self
            .ssm_heads
            .iter()
            .map(|h| h.step(&injected))
            .collect::<Vec<_>>()
            .join(" | ");
        let attn = self
            .attention_heads
            .iter()
            .map(|h| h.attend(&injected))
            .collect::<Vec<_>>()
            .join(" | ");
        format!(
            "[verified deterministic] meta={} ssm=[{}] attn=[{}] kv_stride={}",
            self.config.meta_tokens, ssm, attn, self.config.kv_stride
        )
    }
}
