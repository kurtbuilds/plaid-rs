#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .transfer_metrics_get()
        .originator_client_id("your originator client id")
        .await
        .unwrap();
    println!("{:#?}", response);
}
