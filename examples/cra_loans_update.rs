#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let loans = vec![
        CraLoanUpdate { loan_id : Some("your loan id".to_owned()), payment_history :
        Some(vec![CraLoanPaymentHistory { amount_past_due : Some(1.0), balance_remaining
        : Some(1.0), days_past_due : 1, due_date : chrono::Utc::now().date_naive(),
        period : 1 }]), status_history : Some(vec![CraLoanStatusHistoryUpdate { date :
        chrono::Utc::now().date_naive(), status : CraLoanStatus::Approved }]) }
    ];
    let response = client.cra_loans_update(loans).await.unwrap();
    println!("{:#?}", response);
}
