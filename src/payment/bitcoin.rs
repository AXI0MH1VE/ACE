use super::{PaymentError, PaymentGateway};
use async_trait::async_trait;
use bs58;
use sha3::{Digest, Sha3_256};

#[derive(Clone, Debug)]
pub struct LightningGateway {
    pub node_id: String,
    pub price_per_proof_sats: u64,
}

impl LightningGateway {
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

    fn is_valid_invoice(invoice: &str) -> bool {
        let prefixes = ["lnbc", "lntb", "lnsb"];
        prefixes.iter().any(|prefix| invoice.starts_with(prefix)) && invoice.len() > 10
    }
}

#[async_trait]
impl PaymentGateway for LightningGateway {
    async fn validate_invoice(
        &self,
        invoice: Option<&str>,
        require_payment: bool,
        free_local: bool,
    ) -> Result<(), PaymentError> {
        if !require_payment || free_local {
            return Ok(());
        }

        match invoice {
            Some(raw) if Self::is_valid_invoice(raw) => Ok(()),
            _ => Err(PaymentError::MissingOrInvalidInvoice),
        }
    }
}
