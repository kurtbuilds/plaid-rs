#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .transfer_get()
        .authorization_id("your authorization id")
        .originator_client_id("your originator client id")
        .transfer_id("your transfer id")
        .await
        .unwrap();
    println!("{:#?}", response);
}
