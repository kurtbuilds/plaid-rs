#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let public_token = "your public token";
    let response = client.user_account_session_get(public_token).await.unwrap();
    println!("{:#?}", response);
}
