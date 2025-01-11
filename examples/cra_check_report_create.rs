#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
use plaid::request::cra_check_report_create::CraCheckReportCreateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let consumer_report_permissible_purpose = ConsumerReportPermissiblePurpose::AccountReviewCredit;
    let days_requested = 1;
    let user_token = "your user token";
    let webhook = "your webhook";
    let response = client
        .cra_check_report_create(CraCheckReportCreateRequired {
            consumer_report_permissible_purpose,
            days_requested,
            user_token,
            webhook,
        })
        .days_required(1)
        .await
        .unwrap();
    println!("{:#?}", response);
}
