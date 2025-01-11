#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let amount = "your amount";
    let idempotency_key = "your idempotency key";
    let network = TransferNetwork::Ach;
    let response = client
        .transfer_ledger_withdraw(amount, idempotency_key, network)
        .description("your description")
        .funding_account_id("your funding account id")
        .ledger_id("your ledger id")
        .originator_client_id("your originator client id")
        .await
        .unwrap();
    println!("{:#?}", response);
}
