#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let new_virtual_time = chrono::Utc::now();
    let test_clock_id = "your test clock id";
    let response = client
        .sandbox_transfer_test_clock_advance(new_virtual_time, test_clock_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}
