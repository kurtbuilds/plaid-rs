#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let recipient_id = "your recipient id";
    let response = client.payment_initiation_recipient_get(recipient_id).await.unwrap();
    println!("{:#?}", response);
}
