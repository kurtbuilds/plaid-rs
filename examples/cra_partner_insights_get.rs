#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let user_token = "your user token";
    let response = client.cra_partner_insights_get(user_token).await.unwrap();
    println!("{:#?}", response);
}