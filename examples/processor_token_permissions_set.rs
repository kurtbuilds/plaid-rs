#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let processor_token = "your processor token";
    let products = vec![Products::Assets];
    let response = client
        .processor_token_permissions_set(processor_token, products)
        .await
        .unwrap();
    println!("{:#?}", response);
}
