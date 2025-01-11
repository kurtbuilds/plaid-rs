#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let recurring_transfer_id = "your recurring transfer id";
    let response = client.transfer_recurring_get(recurring_transfer_id).await.unwrap();
    println!("{:#?}", response);
}
