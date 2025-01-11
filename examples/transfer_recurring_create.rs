#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
use plaid::request::TransferRecurringCreateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let account_id = "your account id";
    let amount = "your amount";
    let description = "your description";
    let idempotency_key = "your idempotency key";
    let network = TransferRecurringNetwork::Ach;
    let schedule = TransferRecurringSchedule {
        end_date: Some(chrono::Utc::now().date_naive()),
        interval_count: 1,
        interval_execution_day: 1,
        interval_unit: TransferScheduleIntervalUnit::Week,
        start_date: chrono::Utc::now().date_naive(),
    };
    let type_ = TransferType::Debit;
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
        .transfer_recurring_create(TransferRecurringCreateRequired {
            access_token,
            account_id,
            amount,
            description,
            idempotency_key,
            network,
            schedule,
            type_,
            user,
        })
        .ach_class(AchClass::Ccd)
        .device(TransferDevice {
            ip_address: "your ip address".to_owned(),
            user_agent: "your user agent".to_owned(),
        })
        .funding_account_id("your funding account id")
        .iso_currency_code("your iso currency code")
        .test_clock_id("your test clock id")
        .user_present(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
