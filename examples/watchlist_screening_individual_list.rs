#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let watchlist_program_id = "your watchlist program id";
    let response = client
        .watchlist_screening_individual_list(watchlist_program_id)
        .assignee("your assignee")
        .client_user_id("your client user id")
        .cursor("your cursor")
        .status(WatchlistScreeningStatus::Rejected)
        .await
        .unwrap();
    println!("{:#?}", response);
}
