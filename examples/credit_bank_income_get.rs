#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .credit_bank_income_get()
        .options(CreditBankIncomeGetRequestOptions {
            count: Some(1),
        })
        .user_token("your user token")
        .await
        .unwrap();
    println!("{:#?}", response);
}
