#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
use plaid::request::item_application_scopes_update::ItemApplicationScopesUpdateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let application_id = "your application id";
    let context = ScopesContext::Enrollment;
    let scopes = Scopes {
        accounts: Some(
            vec![
                AccountAccess { account_product_access : Some(AccountProductAccess {
                account_data : Some(true), statements : Some(true), tax_documents :
                Some(true) }), authorized : Some(true), unique_id : "your unique id"
                .to_owned() }
            ],
        ),
        new_accounts: Some(true),
        product_access: Some(ProductAccess {
            accounts_details_transactions: Some(true),
            accounts_routing_number: Some(true),
            accounts_statements: Some(true),
            accounts_tax_statements: Some(true),
            auth: Some(true),
            customers_profiles: Some(true),
            identity: Some(true),
            statements: Some(true),
            transactions: Some(true),
        }),
    };
    let response = client
        .item_application_scopes_update(ItemApplicationScopesUpdateRequired {
            access_token,
            application_id,
            context,
            scopes,
        })
        .state("your state")
        .await
        .unwrap();
    println!("{:#?}", response);
}
