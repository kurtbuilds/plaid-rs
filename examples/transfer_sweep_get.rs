#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let sweep_id = "your sweep id";
    let response = client.transfer_sweep_get(sweep_id).await.unwrap();
    println!("{:#?}", response);
}
