#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client.transfer_originator_list().count(1).offset(1).await.unwrap();
    println!("{:#?}", response);
}
