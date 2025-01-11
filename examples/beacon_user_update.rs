#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let beacon_user_id = "your beacon user id";
    let response = client
        .beacon_user_update(beacon_user_id)
        .access_tokens(&["your access tokens"])
        .user(BeaconUserUpdateRequestData {
            address: Some(BeaconUserRequestAddressNullable {
                city: "your city".to_owned(),
                country: "your country".to_owned(),
                postal_code: Some("your postal code".to_owned()),
                region: Some("your region".to_owned()),
                street: "your street".to_owned(),
                street2: Some("your street 2".to_owned()),
            }),
            date_of_birth: Some(chrono::Utc::now().date_naive()),
            depository_accounts: Some(
                vec![
                    BeaconUserRequestDepositoryAccount { account_number :
                    "your account number".to_owned(), routing_number :
                    "your routing number".to_owned() }
                ],
            ),
            email_address: Some("your email address".to_owned()),
            id_number: Some(BeaconUserIdNumber {
                type_: IdNumberType::ArDni,
                value: "your value".to_owned(),
            }),
            ip_address: Some("your ip address".to_owned()),
            name: Some(BeaconUserNameNullable {
                family_name: "your family name".to_owned(),
                given_name: "your given name".to_owned(),
            }),
            phone_number: Some("your phone number".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
