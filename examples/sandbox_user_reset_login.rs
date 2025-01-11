#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let user_token = "your user token";
    let response = client
        .sandbox_user_reset_login(user_token)
        .item_ids(&["your item ids"])
        .await
        .unwrap();
    println!("{:#?}", response);
}
