#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let parsing_config = vec![IncomeVerificationDocParsingConfig::Ocr];
    let user_token = "your user token";
    let response = client
        .credit_payroll_income_parsing_config_update(parsing_config, user_token)
        .item_id("your item id")
        .await
        .unwrap();
    println!("{:#?}", response);
}
