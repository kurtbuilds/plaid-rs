#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
use plaid::request::signal_evaluate::SignalEvaluateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let account_id = "your account id";
    let amount = 1.0;
    let client_transaction_id = "your client transaction id";
    let response = client
        .signal_evaluate(SignalEvaluateRequired {
            access_token,
            account_id,
            amount,
            client_transaction_id,
        })
        .client_user_id("your client user id")
        .default_payment_method("your default payment method")
        .device(SignalDevice {
            ip_address: Some("your ip address".to_owned()),
            user_agent: Some("your user agent".to_owned()),
        })
        .is_recurring(true)
        .risk_profile_key("your risk profile key")
        .ruleset_key("your ruleset key")
        .user(SignalUser {
            address: Some(SignalAddressData {
                city: Some("your city".to_owned()),
                country: Some("your country".to_owned()),
                postal_code: Some("your postal code".to_owned()),
                region: Some("your region".to_owned()),
                street: Some("your street".to_owned()),
            }),
            email_address: Some("your email address".to_owned()),
            name: Some(SignalPersonName {
                family_name: Some("your family name".to_owned()),
                given_name: Some("your given name".to_owned()),
                middle_name: Some("your middle name".to_owned()),
                prefix: Some("your prefix".to_owned()),
                suffix: Some("your suffix".to_owned()),
            }),
            phone_number: Some("your phone number".to_owned()),
        })
        .user_present(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
