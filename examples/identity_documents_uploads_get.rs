#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let response = client
        .identity_documents_uploads_get(access_token)
        .client_id("your client id")
        .options(IdentityDocumentsUploadsGetRequestOptions {
            account_ids: Some(vec!["your account ids".to_owned()]),
        })
        .secret("your secret")
        .await
        .unwrap();
    println!("{:#?}", response);
}
