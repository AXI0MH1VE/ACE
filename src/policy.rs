use serde::Deserialize;
use std::{fs, path::Path};
use thiserror::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct SafetyConfig {
    pub allow_network: bool,
    pub allow_verified: bool,
    pub blocklist: Vec<String>,
    pub high_risk_terms: Vec<String>,
}

impl Default for SafetyConfig {
    fn default() -> Self {
        Self {
            allow_network: false,
            allow_verified: true,
            blocklist: vec![
                "self-harm".into(),
                "bioweapon".into(),
                "malware".into(),
                "child exploitation".into(),
                "terrorism".into(),
            ],
            high_risk_terms: vec![
                "financial advice".into(),
                "medical diagnosis".into(),
                "legal judgment".into(),
            ],
        }
    }
}

#[derive(Debug, Error)]
pub enum PolicyError {
    #[error("blocked content: {0}")]
    Blocked(String),
    #[error("verified mode disabled")]
    VerifiedDisabled,
    #[error("network disabled for this request")]
    NetworkDisabled,
    #[error("high-risk content requires consent")]
    EscalationRequired(String),
    #[error("failed to load safety config: {0}")]
    ConfigLoad(String),
}

#[derive(Debug, Clone)]
pub struct PolicyVerdict {
    pub high_risk_terms: Vec<String>,
}

#[derive(Clone)]
pub struct SafetyPolicy {
    config: SafetyConfig,
}

impl SafetyPolicy {
    pub fn load_from_disk(path: impl AsRef<Path>) -> Result<Self, PolicyError> {
        let config = match fs::read_to_string(path) {
            Ok(raw) => serde_json::from_str::<SafetyConfig>(&raw)
                .map_err(|e| PolicyError::ConfigLoad(e.to_string()))?,
            Err(_) => SafetyConfig::default(),
        };
        Ok(Self { config })
    }

    pub fn check_prompt(&self, prompt: &str) -> Result<PolicyVerdict, PolicyError> {
        let lower = prompt.to_lowercase();
        if let Some(term) = self
            .config
            .blocklist
            .iter()
            .find(|term| lower.contains(term.as_str()))
        {
            return Err(PolicyError::Blocked(term.clone()));
        }

        let high_risk_terms = self
            .config
            .high_risk_terms
            .iter()
            .filter(|term| lower.contains(term.as_str()))
            .cloned()
            .collect::<Vec<_>>();

        if !high_risk_terms.is_empty() {
            return Err(PolicyError::EscalationRequired(high_risk_terms.join(", ")));
        }

        Ok(PolicyVerdict { high_risk_terms })
    }

    pub fn ensure_verified_enabled(&self) -> Result<(), PolicyError> {
        if !self.config.allow_verified {
            return Err(PolicyError::VerifiedDisabled);
        }
        Ok(())
    }

    pub fn ensure_network_allowed(&self, request_flag: bool) -> Result<(), PolicyError> {
        if request_flag && !self.config.allow_network {
            return Err(PolicyError::NetworkDisabled);
        }
        Ok(())
    }

    pub fn allow_network_by_default(&self) -> bool {
        self.config.allow_network
    }
}
