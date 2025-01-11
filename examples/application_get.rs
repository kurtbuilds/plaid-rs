#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let application_id = "your application id";
    let response = client.application_get(application_id).await.unwrap();
    println!("{:#?}", response);
}
