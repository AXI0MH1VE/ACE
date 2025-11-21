use async_trait::async_trait;
use thiserror::Error;

pub mod bitcoin;

#[derive(Debug, Error)]
pub enum PaymentError {
    #[error("missing or invalid Lightning invoice")]
    MissingOrInvalidInvoice,
    #[error("payment not required")]
    PaymentNotRequired,
}

#[async_trait]
pub trait PaymentGateway: Send + Sync {
    async fn validate_invoice(
        &self,
        invoice: Option<&str>,
        require_payment: bool,
        free_local: bool,
    ) -> Result<(), PaymentError>;
}
