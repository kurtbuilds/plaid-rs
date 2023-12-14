#![allow(unused_imports)]
use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let client_user_id = "your client user id";
    let program_id = "your program id";
    let user = BeaconUserRequestData {
        address: BeaconUserRequestAddress {
            city: "your city".to_owned(),
            country: "your country".to_owned(),
            postal_code: Some("your postal code".to_owned()),
            region: Some("your region".to_owned()),
            street: "your street".to_owned(),
            street2: Some("your street 2".to_owned()),
        },
        date_of_birth: chrono::Utc::now().date_naive(),
        email_address: Some("your email address".to_owned()),
        id_number: Some(BeaconUserIdNumber {
            type_: "your type".to_owned(),
            value: "your value".to_owned(),
        }),
        ip_address: Some("your ip address".to_owned()),
        name: BeaconUserName {
            family_name: "your family name".to_owned(),
            given_name: "your given name".to_owned(),
        },
        phone_number: Some("your phone number".to_owned()),
    };
    let response = client
        .beacon_user_create(client_user_id, program_id, user)
        .await
        .unwrap();
    println!("{:#?}", response);
}