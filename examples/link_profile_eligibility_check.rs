#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let link_session_id = "your link session id";
    let user = LinkProfileEligibilityCheckUser {
        phone_number: "your phone number".to_owned(),
    };
    let response = client
        .link_profile_eligibility_check(link_session_id, user)
        .await
        .unwrap();
    println!("{:#?}", response);
}
