#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let event_type = TransferLedgerSweepSimulateEventType::SweepPosted;
    let sweep_id = "your sweep id";
    let response = client
        .sandbox_transfer_ledger_deposit_simulate(event_type, sweep_id)
        .failure_reason(TransferFailure {
            ach_return_code: Some("your ach return code".to_owned()),
            description: Some("your description".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
