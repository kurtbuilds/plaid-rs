#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
use plaid::request::transfer_create::TransferCreateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let account_id = "your account id";
    let authorization_id = "your authorization id";
    let description = "your description";
    let response = client
        .transfer_create(TransferCreateRequired {
            access_token,
            account_id,
            authorization_id,
            description,
        })
        .ach_class(AchClass::Ccd)
        .amount("your amount")
        .facilitator_fee("your facilitator fee")
        .idempotency_key("your idempotency key")
        .iso_currency_code("your iso currency code")
        .metadata(std::collections::HashMap::new())
        .network(TransferNetwork::Ach)
        .origination_account_id("your origination account id")
        .test_clock_id("your test clock id")
        .type_(TransferType::Debit)
        .user(TransferUserInRequestDeprecated {
            address: Some(TransferUserAddressInRequest {
                city: Some("your city".to_owned()),
                country: Some("your country".to_owned()),
                postal_code: Some("your postal code".to_owned()),
                region: Some("your region".to_owned()),
                street: Some("your street".to_owned()),
            }),
            email_address: Some("your email address".to_owned()),
            legal_name: Some("your legal name".to_owned()),
            phone_number: Some("your phone number".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
