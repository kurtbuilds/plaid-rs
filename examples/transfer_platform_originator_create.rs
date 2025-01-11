#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let originator_client_id = "your originator client id";
    let originator_reviewed_at = chrono::Utc::now();
    let tos_acceptance_metadata = TransferPlatformTosAcceptanceMetadata {
        agreement_accepted: true,
        agreement_accepted_at: chrono::Utc::now(),
        originator_ip_address: "your originator ip address".to_owned(),
    };
    let response = client
        .transfer_platform_originator_create(
            originator_client_id,
            originator_reviewed_at,
            tos_acceptance_metadata,
        )
        .await
        .unwrap();
    println!("{:#?}", response);
}
