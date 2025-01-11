#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let third_party_user_token = "your third party user token";
    let response = client
        .user_third_party_token_remove(third_party_user_token)
        .await
        .unwrap();
    println!("{:#?}", response);
}
