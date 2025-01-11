#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .income_verification_paystubs_get()
        .access_token("your access token")
        .income_verification_id("your income verification id")
        .await
        .unwrap();
    println!("{:#?}", response);
}
