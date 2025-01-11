#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let body = serde_json::json!({});
    let response = client.payment_initiation_recipient_list(body).await.unwrap();
    println!("{:#?}", response);
}
