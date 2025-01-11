#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .cra_check_report_base_report_get()
        .item_ids(&["your item ids"])
        .third_party_user_token("your third party user token")
        .user_token("your user token")
        .await
        .unwrap();
    println!("{:#?}", response);
}
