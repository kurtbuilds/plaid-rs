#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let loans = vec![
        CraLoanRegister { application : Some(CraLoanRegisterApplication {
        application_date : Some(chrono::Utc::now().date_naive()), application_id :
        Some("your application id".to_owned()) }), loan_amount : Some(1.0), loan_id :
        "your loan id".to_owned(), opened_date : chrono::Utc::now().date_naive(),
        opened_with_status : CraLoanOpenedStatus { date : chrono::Utc::now()
        .date_naive(), status : CraLoanStatus::Approved }, payment_schedule :
        CraLoanPaymentSchedule::Daily, type_ : CraLoanType::Personal, user_token :
        "your user token".to_owned() }
    ];
    let response = client.cra_loans_register(loans).await.unwrap();
    println!("{:#?}", response);
}
