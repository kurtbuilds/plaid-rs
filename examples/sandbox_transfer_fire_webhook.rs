#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let webhook = "your webhook";
    let response = client.sandbox_transfer_fire_webhook(webhook).await.unwrap();
    println!("{:#?}", response);
}
