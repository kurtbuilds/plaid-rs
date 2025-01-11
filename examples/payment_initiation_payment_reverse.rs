#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let idempotency_key = "your idempotency key";
    let payment_id = "your payment id";
    let reference = "your reference";
    let response = client
        .payment_initiation_payment_reverse(idempotency_key, payment_id, reference)
        .amount(PaymentAmountNullable {
            currency: PaymentAmountCurrency::Gbp,
            value: 1.0,
        })
        .counterparty_address(PaymentInitiationAddress {
            city: "your city".to_owned(),
            country: "your country".to_owned(),
            postal_code: "your postal code".to_owned(),
            street: vec!["your street".to_owned()],
        })
        .counterparty_date_of_birth(chrono::Utc::now().date_naive())
        .await
        .unwrap();
    println!("{:#?}", response);
}
