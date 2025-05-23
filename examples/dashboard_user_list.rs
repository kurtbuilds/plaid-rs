#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client.dashboard_user_list().cursor("your cursor").await.unwrap();
    println!("{:#?}", response);
}
