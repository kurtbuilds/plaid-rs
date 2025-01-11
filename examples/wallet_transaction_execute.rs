#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
use plaid::request::WalletTransactionExecuteRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let amount = WalletTransactionAmount {
        iso_currency_code: WalletIsoCurrencyCode::Gbp,
        value: 1.0,
    };
    let counterparty = WalletTransactionCounterparty {
        address: Some(PaymentInitiationAddress {
            city: "your city".to_owned(),
            country: "your country".to_owned(),
            postal_code: "your postal code".to_owned(),
            street: vec!["your street".to_owned()],
        }),
        date_of_birth: Some(chrono::Utc::now().date_naive()),
        name: "your name".to_owned(),
        numbers: WalletTransactionCounterpartyNumbers {
            bacs: Some(RecipientBacs {
                account: Some("your account".to_owned()),
                sort_code: Some("your sort code".to_owned()),
            }),
            international: Some(WalletTransactionCounterpartyInternational {
                iban: Some("your iban".to_owned()),
            }),
        },
    };
    let idempotency_key = "your idempotency key";
    let reference = "your reference";
    let wallet_id = "your wallet id";
    let response = client
        .wallet_transaction_execute(WalletTransactionExecuteRequired {
            amount,
            counterparty,
            idempotency_key,
            reference,
            wallet_id,
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
