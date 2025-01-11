#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let consumer_report_permissible_purpose = ConsumerReportPermissiblePurpose::AccountReviewCredit;
    let days_requested = 1;
    let user_token = "your user token";
    let response = client
        .cra_base_report_create(
            consumer_report_permissible_purpose,
            days_requested,
            user_token,
        )
        .webhook("your webhook")
        .await
        .unwrap();
    println!("{:#?}", response);
}
