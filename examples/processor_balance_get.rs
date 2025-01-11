#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let processor_token = "your processor token";
    let response = client
        .processor_balance_get(processor_token)
        .options(ProcessorBalanceGetRequestOptions {
            min_last_updated_datetime: Some(chrono::Utc::now()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
