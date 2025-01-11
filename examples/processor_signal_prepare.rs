#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let processor_token = "your processor token";
    let response = client.processor_signal_prepare(processor_token).await.unwrap();
    println!("{:#?}", response);
}
