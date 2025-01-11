#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let user = ProfileNetworkStatusGetUser {
        phone_number: "your phone number".to_owned(),
    };
    let response = client.profile_network_status_get(user).await.unwrap();
    println!("{:#?}", response);
}
