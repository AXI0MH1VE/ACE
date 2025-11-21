use bs58;
use sha3::{Digest, Sha3_256};

#[derive(Clone, Debug)]
pub struct LightningBilling {
    pub node_id: String,
    pub price_per_proof_sats: u64,
}

impl LightningBilling {
    pub fn new(node_id: &str) -> Self {
        Self {
            node_id: node_id.into(),
            price_per_proof_sats: 1_000,
        }
    }

    pub fn invoice_for_task(&self, request_id: &str) -> String {
        let mut hasher = Sha3_256::new();
        hasher.update(request_id.as_bytes());
        let digest = hasher.finalize();
        let encoded = bs58::encode(digest).into_string();
        format!("lnbc{}n1{}", self.price_per_proof_sats, encoded)
    }

    pub fn verify_payment(&self, invoice: &str) -> bool {
        invoice.starts_with("lnbc")
    }
}
