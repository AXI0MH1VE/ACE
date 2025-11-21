use axiomhive::payment::bitcoin::LightningGateway;
use axiomhive::payment::{PaymentError, PaymentGateway};
use futures::executor::block_on;

#[test]
fn invoice_required_when_payment_enabled() {
    let gateway = LightningGateway::new("node");
    let err = block_on(gateway.validate_invoice(None, true, false));
    assert!(matches!(err, Err(PaymentError::MissingOrInvalidInvoice)));

    let ok = block_on(gateway.validate_invoice(Some("lnbc123456789"), true, false));
    assert!(ok.is_ok());
}

#[test]
fn free_local_bypasses_payment() {
    let gateway = LightningGateway::new("node");
    let res = block_on(gateway.validate_invoice(None, true, true));
    assert!(res.is_ok());
}

#[test]
fn payment_disabled_allows_no_invoice() {
    let gateway = LightningGateway::new("node");
    let res = block_on(gateway.validate_invoice(None, false, false));
    assert!(res.is_ok());
}
