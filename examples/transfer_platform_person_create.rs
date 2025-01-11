#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let originator_client_id = "your originator client id";
    let response = client
        .transfer_platform_person_create(originator_client_id)
        .address(TransferPlatformPersonAddress {
            city: "your city".to_owned(),
            country: "your country".to_owned(),
            postal_code: "your postal code".to_owned(),
            region: "your region".to_owned(),
            street: "your street".to_owned(),
            street2: Some("your street 2".to_owned()),
        })
        .date_of_birth(chrono::Utc::now().date_naive())
        .email_address("your email address")
        .id_number(TransferPlatformPersonIdNumber {
            type_: IdNumberType::ArDni,
            value: "your value".to_owned(),
        })
        .name(TransferPlatformPersonName {
            family_name: "your family name".to_owned(),
            given_name: "your given name".to_owned(),
        })
        .phone_number("your phone number")
        .relationship_to_originator("your relationship to originator")
        .await
        .unwrap();
    println!("{:#?}", response);
}
