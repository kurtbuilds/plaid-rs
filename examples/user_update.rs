#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let user_token = "your user token";
    let response = client
        .user_update(user_token)
        .consumer_report_user_identity(ConsumerReportUserIdentity {
            date_of_birth: Some(chrono::Utc::now().date_naive()),
            emails: vec!["your emails".to_owned()],
            first_name: "your first name".to_owned(),
            last_name: "your last name".to_owned(),
            phone_numbers: vec!["your phone numbers".to_owned()],
            primary_address: AddressData {
                city: Some("your city".to_owned()),
                country: Some("your country".to_owned()),
                postal_code: Some("your postal code".to_owned()),
                region: Some("your region".to_owned()),
                street: "your street".to_owned(),
            },
            ssn_last4: Some("your ssn last 4".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
