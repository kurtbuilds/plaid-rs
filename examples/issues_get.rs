#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let issue_id = "your issue id";
    let response = client.issues_get(issue_id).await.unwrap();
    println!("{:#?}", response);
}
