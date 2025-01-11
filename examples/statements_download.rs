#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let statement_id = "your statement id";
    let response = client.statements_download(access_token, statement_id).await.unwrap();
    println!("{:#?}", response);
}
