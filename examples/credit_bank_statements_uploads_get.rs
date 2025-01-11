#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let user_token = "your user token";
    let response = client
        .credit_bank_statements_uploads_get(user_token)
        .options(CreditBankStatementsUploadsGetRequestOptions {
            item_ids: Some(vec!["your item ids".to_owned()]),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
