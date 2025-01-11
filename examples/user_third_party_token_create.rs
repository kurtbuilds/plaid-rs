#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let third_party_client_id = "your third party client id";
    let user_token = "your user token";
    let response = client
        .user_third_party_token_create(third_party_client_id, user_token)
        .expiration_time(chrono::Utc::now())
        .await
        .unwrap();
    println!("{:#?}", response);
}
