#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let accounts = &["your accounts"];
    let oauth_state_id = "your oauth state id";
    let response = client
        .sandbox_oauth_select_accounts(accounts, oauth_state_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}
