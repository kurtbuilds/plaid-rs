#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let loans = vec![
        CraLoanUnregister { closed_with_status : CraLoanClosedStatus { date :
        chrono::Utc::now().date_naive(), status : CraLoanStatus::Approved }, loan_id :
        "your loan id".to_owned() }
    ];
    let response = client.cra_loans_unregister(loans).await.unwrap();
    println!("{:#?}", response);
}
