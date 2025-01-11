#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .transfer_repayment_list()
        .count(1)
        .end_date(chrono::Utc::now())
        .offset(1)
        .start_date(chrono::Utc::now())
        .await
        .unwrap();
    println!("{:#?}", response);
}
