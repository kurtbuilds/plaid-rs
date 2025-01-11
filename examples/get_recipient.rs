#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let recipient_id = "your recipient id";
    let response = client
        .get_recipient(recipient_id)
        .oauth_state_id("your oauth state id")
        .await
        .unwrap();
    println!("{:#?}", response);
}
