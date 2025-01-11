#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let profile_token = "your profile token";
    let response = client.profile_get(profile_token).await.unwrap();
    println!("{:#?}", response);
}
