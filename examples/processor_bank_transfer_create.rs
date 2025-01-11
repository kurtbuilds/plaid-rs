#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
use plaid::request::processor_bank_transfer_create::ProcessorBankTransferCreateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let amount = "your amount";
    let description = "your description";
    let idempotency_key = "your idempotency key";
    let iso_currency_code = "your iso currency code";
    let network = BankTransferNetwork::Ach;
    let processor_token = "your processor token";
    let type_ = BankTransferType::Debit;
    let user = BankTransferUser {
        email_address: Some("your email address".to_owned()),
        legal_name: "your legal name".to_owned(),
        routing_number: Some("your routing number".to_owned()),
    };
    let response = client
        .processor_bank_transfer_create(ProcessorBankTransferCreateRequired {
            amount,
            description,
            idempotency_key,
            iso_currency_code,
            network,
            processor_token,
            type_,
            user,
        })
        .ach_class(AchClass::Ccd)
        .custom_tag("your custom tag")
        .metadata(std::collections::HashMap::new())
        .origination_account_id("your origination account id")
        .await
        .unwrap();
    println!("{:#?}", response);
}
