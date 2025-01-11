#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let relay_token = "your relay token";
    let report_type = ReportType::Asset;
    let response = client.credit_relay_pdf_get(relay_token, report_type).await.unwrap();
    println!("{:#?}", response);
}
