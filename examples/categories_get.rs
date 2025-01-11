#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let body = serde_json::json!({});
    let response = client.categories_get(body).await.unwrap();
    println!("{:#?}", response);
}
