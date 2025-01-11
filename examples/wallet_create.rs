#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let iso_currency_code = WalletIsoCurrencyCode::Gbp;
    let response = client.wallet_create(iso_currency_code).await.unwrap();
    println!("{:#?}", response);
}
