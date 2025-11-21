#[derive(Clone, Debug)]
pub struct ZkmlConfig {
    pub prover: String,
    pub circuit_size: usize,
}

impl Default for ZkmlConfig {
    fn default() -> Self {
        Self {
            prover: "ezkl-halo2".into(),
            circuit_size: 2_usize.pow(16),
        }
    }
}

pub struct ZkmlProofEngine {
    pub config: ZkmlConfig,
}

impl ZkmlProofEngine {
    pub fn new(config: ZkmlConfig) -> Self {
        Self { config }
    }

    pub fn prove(&self, statement: &str) -> String {
        format!(
            "zkml_proof(prover={}, n={}) for '{}'",
            self.config.prover, self.config.circuit_size, statement
        )
    }
}
