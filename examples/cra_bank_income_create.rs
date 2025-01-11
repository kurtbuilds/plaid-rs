#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .cra_bank_income_create()
        .consumer_report_permissible_purpose(
            ConsumerReportPermissiblePurpose::AccountReviewCredit,
        )
        .days_requested(1)
        .user_token("your user token")
        .webhook("your webhook")
        .await
        .unwrap();
    println!("{:#?}", response);
}
