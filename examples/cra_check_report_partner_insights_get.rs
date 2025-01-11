#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .cra_check_report_partner_insights_get()
        .options(CraCheckReportPartnerInsightsGetOptions {
            prism_products: Some(vec![PrismProduct::Insights]),
            prism_versions: Some(PrismVersions {
                cashscore: Some(PrismCashScoreVersion(serde_json::json!({}))),
                firstdetect: Some(PrismFirstDetectVersion(serde_json::json!({}))),
                insights: Some(PrismInsightsVersion(serde_json::json!({}))),
            }),
        })
        .third_party_user_token("your third party user token")
        .user_token("your user token")
        .await
        .unwrap();
    println!("{:#?}", response);
}
