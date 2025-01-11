#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .beacon_account_risk_evaluate()
        .access_token("your access token")
        .client_evaluation_id("your client evaluation id")
        .client_user_id("your client user id")
        .device(SignalDevice {
            ip_address: Some("your ip address".to_owned()),
            user_agent: Some("your user agent".to_owned()),
        })
        .evaluate_time("your evaluate time")
        .evaluation_reason(BeaconAccountRiskEvaluateEvaluationReason::Onboarding)
        .options(BeaconAccountRiskEvaluateRequestOptions {
            account_ids: Some(vec!["your account ids".to_owned()]),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
