#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let authorization_id = "your authorization id";
    let response = client.transfer_authorization_cancel(authorization_id).await.unwrap();
    println!("{:#?}", response);
}
