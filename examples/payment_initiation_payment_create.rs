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
    let recipient_id = "your recipient id";
    let reference = "your reference";
    let response = client
        .payment_initiation_payment_create(amount, recipient_id, reference)
        .options(ExternalPaymentOptions {
            bacs: Some(RecipientBacs {
                account: Some("your account".to_owned()),
                sort_code: Some("your sort code".to_owned()),
            }),
            iban: Some("your iban".to_owned()),
            request_refund_details: Some(true),
            scheme: Some(PaymentScheme(serde_json::json!({}))),
        })
        .schedule(ExternalPaymentScheduleBase {
            adjusted_start_date: Some(chrono::Utc::now().date_naive()),
            end_date: Some(chrono::Utc::now().date_naive()),
            interval: Some(PaymentScheduleInterval::Weekly),
            interval_execution_day: Some(1),
            start_date: Some(chrono::Utc::now().date_naive()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
