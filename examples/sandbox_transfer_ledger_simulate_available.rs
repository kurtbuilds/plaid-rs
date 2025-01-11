#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .sandbox_transfer_ledger_simulate_available()
        .ledger_id("your ledger id")
        .test_clock_id("your test clock id")
        .webhook("your webhook")
        .await
        .unwrap();
    println!("{:#?}", response);
}
