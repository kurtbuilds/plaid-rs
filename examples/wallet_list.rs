#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .wallet_list()
        .count(1)
        .cursor("your cursor")
        .iso_currency_code(WalletIsoCurrencyCode::Gbp)
        .await
        .unwrap();
    println!("{:#?}", response);
}
