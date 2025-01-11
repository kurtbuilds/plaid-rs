#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let constraints = PaymentInitiationConsentConstraints {
        max_payment_amount: PaymentAmount {
            currency: PaymentAmountCurrency::Gbp,
            value: 1.0,
        },
        periodic_amounts: vec![
            PaymentConsentPeriodicAmount { alignment :
            PaymentConsentPeriodicAlignment::Calendar, amount : PaymentAmount { currency
            : PaymentAmountCurrency::Gbp, value : 1.0 }, interval :
            PaymentConsentPeriodicInterval::Day }
        ],
        valid_date_time: Some(PaymentConsentValidDateTime {
            from: Some(chrono::Utc::now()),
            to: Some(chrono::Utc::now()),
        }),
    };
    let recipient_id = "your recipient id";
    let reference = "your reference";
    let response = client
        .payment_initiation_consent_create(constraints, recipient_id, reference)
        .options(ExternalPaymentInitiationConsentOptions {
            bacs: Some(RecipientBacs {
                account: Some("your account".to_owned()),
                sort_code: Some("your sort code".to_owned()),
            }),
            iban: Some("your iban".to_owned()),
            request_refund_details: Some(true),
        })
        .payer_details(PaymentInitiationConsentPayerDetails {
            address: Some(PaymentInitiationAddress {
                city: "your city".to_owned(),
                country: "your country".to_owned(),
                postal_code: "your postal code".to_owned(),
                street: vec!["your street".to_owned()],
            }),
            date_of_birth: Some(chrono::Utc::now().date_naive()),
            emails: Some(vec!["your emails".to_owned()]),
            name: "your name".to_owned(),
            numbers: PaymentInitiationConsentPayerNumbers {
                bacs: Some(RecipientBacs {
                    account: Some("your account".to_owned()),
                    sort_code: Some("your sort code".to_owned()),
                }),
                iban: Some("your iban".to_owned()),
            },
            phone_numbers: Some(vec!["your phone numbers".to_owned()]),
        })
        .scopes(vec![PaymentInitiationConsentScope::MeToMe])
        .type_(PaymentInitiationConsentType::Sweeping)
        .await
        .unwrap();
    println!("{:#?}", response);
}
