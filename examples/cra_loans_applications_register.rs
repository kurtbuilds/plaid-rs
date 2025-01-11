#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let applications = vec![
        CraLoanApplication { application_date : Some(chrono::Utc::now().date_naive()),
        application_id : "your application id".to_owned(), decision :
        CraLoanApplicationDecision::Approved, decision_date : Some(chrono::Utc::now()
        .date_naive()), type_ : CraLoanType::Personal, user_token : "your user token"
        .to_owned() }
    ];
    let response = client.cra_loans_applications_register(applications).await.unwrap();
    println!("{:#?}", response);
}
