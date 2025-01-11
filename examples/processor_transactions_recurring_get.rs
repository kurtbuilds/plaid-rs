#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let processor_token = "your processor token";
    let response = client
        .processor_transactions_recurring_get(processor_token)
        .options(TransactionsRecurringGetRequestOptions {
            include_personal_finance_category: Some(true),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
