#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let originator_client_id = "your originator client id";
    let requirement_submissions = vec![
        TransferPlatformRequirementSubmission { person_id : Some("your person id"
        .to_owned()), requirement_type : "your requirement type".to_owned(), value :
        "your value".to_owned() }
    ];
    let response = client
        .transfer_platform_requirement_submit(
            originator_client_id,
            requirement_submissions,
        )
        .await
        .unwrap();
    println!("{:#?}", response);
}
