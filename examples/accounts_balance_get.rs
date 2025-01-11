#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let response = client
        .accounts_balance_get(access_token)
        .options(AccountsBalanceGetRequestOptions {
            account_ids: Some(vec!["your account ids".to_owned()]),
            min_last_updated_datetime: Some(chrono::Utc::now()),
        })
        .payment_details(AccountsBalanceGetRequestPaymentDetails {
            account_id: "your account id".to_owned(),
            amount: 1.0,
            balance_threshold_percentage: Some(1),
            client_transaction_id: "your client transaction id".to_owned(),
            requires_real_time_balance_refresh: Some(true),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
