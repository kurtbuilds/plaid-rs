#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .issues_search()
        .item_id("your item id")
        .link_session_id("your link session id")
        .link_session_request_id("your link session request id")
        .await
        .unwrap();
    println!("{:#?}", response);
}
