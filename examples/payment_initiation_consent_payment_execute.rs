#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let amount = PaymentAmount {
        currency: PaymentAmountCurrency::Gbp,
        value: 1.0,
    };
    let consent_id = "your consent id";
    let idempotency_key = "your idempotency key";
    let response = client
        .payment_initiation_consent_payment_execute(amount, consent_id, idempotency_key)
        .processing_mode(PaymentInitiationConsentProcessingMode::Async)
        .reference("your reference")
        .scope(serde_json::json!({}))
        .await
        .unwrap();
    println!("{:#?}", response);
}
