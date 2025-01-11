#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let refund_id = "your refund id";
    let response = client.transfer_refund_get(refund_id).await.unwrap();
    println!("{:#?}", response);
}
