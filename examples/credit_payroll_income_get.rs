#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .credit_payroll_income_get()
        .options(CreditPayrollIncomeGetRequestOptions {
            item_ids: Some(vec!["your item ids".to_owned()]),
        })
        .user_token("your user token")
        .await
        .unwrap();
    println!("{:#?}", response);
}
