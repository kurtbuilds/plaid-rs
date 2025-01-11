#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let beacon_user_id = "your beacon user id";
    let fraud_date = chrono::Utc::now().date_naive();
    let type_ = BeaconReportCreateType::FirstParty;
    let response = client
        .beacon_report_create(beacon_user_id, fraud_date, type_)
        .fraud_amount(FraudAmount {
            iso_currency_code: IsoCurrencyCode::Usd,
            value: 1.0,
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
