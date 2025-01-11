#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let subscription_id = "your subscription id";
    let response = client
        .cra_monitoring_insights_unsubscribe(subscription_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}
