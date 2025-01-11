#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let entity_watchlist_program_id = "your entity watchlist program id";
    let response = client
        .watchlist_screening_entity_program_get(entity_watchlist_program_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}
