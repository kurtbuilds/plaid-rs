#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let beacon_report_syndication_id = "your beacon report syndication id";
    let response = client
        .beacon_report_syndication_get(beacon_report_syndication_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}
