#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
use plaid::request::TransferLedgerDistributeRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let amount = "your amount";
    let from_ledger_id = "your from ledger id";
    let idempotency_key = "your idempotency key";
    let to_ledger_id = "your to ledger id";
    let response = client
        .transfer_ledger_distribute(TransferLedgerDistributeRequired {
            amount,
            from_ledger_id,
            idempotency_key,
            to_ledger_id,
        })
        .description("your description")
        .await
        .unwrap();
    println!("{:#?}", response);
}
