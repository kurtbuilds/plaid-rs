#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
use plaid::request::transfer_intent_create::TransferIntentCreateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let amount = "your amount";
    let description = "your description";
    let mode = TransferIntentCreateMode::Payment;
    let user = TransferUserInRequest {
        address: Some(TransferUserAddressInRequest {
            city: Some("your city".to_owned()),
            country: Some("your country".to_owned()),
            postal_code: Some("your postal code".to_owned()),
            region: Some("your region".to_owned()),
            street: Some("your street".to_owned()),
        }),
        email_address: Some("your email address".to_owned()),
        legal_name: "your legal name".to_owned(),
        phone_number: Some("your phone number".to_owned()),
    };
    let response = client
        .transfer_intent_create(TransferIntentCreateRequired {
            amount,
            description,
            mode,
            user,
        })
        .account_id("your account id")
        .ach_class(AchClass::Ccd)
        .funding_account_id("your funding account id")
        .iso_currency_code("your iso currency code")
        .metadata(std::collections::HashMap::new())
        .network(TransferIntentCreateNetwork::Ach)
        .origination_account_id("your origination account id")
        .require_guarantee(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
