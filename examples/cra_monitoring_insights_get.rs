#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let consumer_report_permissible_purpose = MonitoringConsumerReportPermissiblePurpose::AccountReviewCredit;
    let user_token = "your user token";
    let response = client
        .cra_monitoring_insights_get(consumer_report_permissible_purpose, user_token)
        .await
        .unwrap();
    println!("{:#?}", response);
}
