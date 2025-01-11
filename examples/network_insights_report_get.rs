#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_tokens = &["your access tokens"];
    let response = client.network_insights_report_get(access_tokens).await.unwrap();
    println!("{:#?}", response);
}
