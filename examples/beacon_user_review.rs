#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let beacon_user_id = "your beacon user id";
    let status = BeaconUserStatus::Rejected;
    let response = client.beacon_user_review(beacon_user_id, status).await.unwrap();
    println!("{:#?}", response);
}
