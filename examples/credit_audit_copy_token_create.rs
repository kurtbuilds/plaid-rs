#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let report_tokens = &["your report tokens"];
    let response = client.credit_audit_copy_token_create(report_tokens).await.unwrap();
    println!("{:#?}", response);
}
