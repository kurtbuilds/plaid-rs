#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let entity_watchlist_program_id = "your entity watchlist program id";
    let response = client
        .watchlist_screening_entity_list(entity_watchlist_program_id)
        .assignee("your assignee")
        .client_user_id("your client user id")
        .cursor("your cursor")
        .status(WatchlistScreeningStatus::Rejected)
        .await
        .unwrap();
    println!("{:#?}", response);
}
