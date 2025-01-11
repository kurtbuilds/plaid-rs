#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
use plaid::request::TransferAuthorizationCreateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let account_id = "your account id";
    let amount = "your amount";
    let network = TransferNetwork::Ach;
    let type_ = TransferType::Debit;
    let user = TransferAuthorizationUserInRequest {
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
        .transfer_authorization_create(TransferAuthorizationCreateRequired {
            access_token,
            account_id,
            amount,
            network,
            type_,
            user,
        })
        .ach_class(AchClass::Ccd)
        .beacon_session_id("your beacon session id")
        .credit_funds_source(TransferCreditFundsSource(serde_json::json!({})))
        .device(TransferAuthorizationDevice {
            ip_address: Some("your ip address".to_owned()),
            user_agent: Some("your user agent".to_owned()),
        })
        .funding_account_id("your funding account id")
        .idempotency_key("your idempotency key")
        .iso_currency_code("your iso currency code")
        .ledger_id("your ledger id")
        .origination_account_id("your origination account id")
        .originator_client_id("your originator client id")
        .payment_profile_token("your payment profile token")
        .test_clock_id("your test clock id")
        .user_present(true)
        .wire_details(TransferWireDetails {
            message_to_beneficiary: Some("your message to beneficiary".to_owned()),
        })
        .with_guarantee(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
