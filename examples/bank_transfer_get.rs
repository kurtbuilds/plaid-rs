#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let bank_transfer_id = "your bank transfer id";
    let response = client.bank_transfer_get(bank_transfer_id).await.unwrap();
    println!("{:#?}", response);
}
