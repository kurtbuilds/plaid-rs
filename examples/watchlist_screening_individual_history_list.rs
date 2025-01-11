#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let watchlist_screening_id = "your watchlist screening id";
    let response = client
        .watchlist_screening_individual_history_list(watchlist_screening_id)
        .cursor("your cursor")
        .await
        .unwrap();
    println!("{:#?}", response);
}
