#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let user_token = "your user token";
    let webhook = "your webhook";
    let response = client
        .cra_monitoring_insights_subscribe(user_token, webhook)
        .await
        .unwrap();
    println!("{:#?}", response);
}
