#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let personal_finance_category = "your personal finance category";
    let rule_details = TransactionsRuleDetails {
        field: TransactionsRuleField::TransactionId,
        query: "your query".to_owned(),
        type_: TransactionsRuleType::ExactMatch,
    };
    let response = client
        .transactions_rules_create(access_token, personal_finance_category, rule_details)
        .await
        .unwrap();
    println!("{:#?}", response);
}
