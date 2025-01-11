#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .transfer_ledger_get()
        .ledger_id("your ledger id")
        .originator_client_id("your originator client id")
        .await
        .unwrap();
    println!("{:#?}", response);
}
