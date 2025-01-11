#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let products = &["your products"];
    let query = "your query";
    let response = client.employers_search(products, query).await.unwrap();
    println!("{:#?}", response);
}
