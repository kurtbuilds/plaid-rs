#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .transfer_balance_get()
        .originator_client_id("your originator client id")
        .type_(TransferBalanceType::PrefundedRtpCredits)
        .await
        .unwrap();
    println!("{:#?}", response);
}
