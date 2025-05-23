use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{
    AchClass, BankTransferMetadata, BankTransferNetwork, BankTransferType,
    BankTransferUser,
};
/**You should use this struct via [`PlaidClient::bank_transfer_create`].

On request success, this will return a [`BankTransferCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferCreateRequest {
    pub access_token: String,
    pub account_id: String,
    pub ach_class: Option<AchClass>,
    pub amount: String,
    pub custom_tag: Option<String>,
    pub description: String,
    pub idempotency_key: String,
    pub iso_currency_code: String,
    pub metadata: Option<BankTransferMetadata>,
    pub network: BankTransferNetwork,
    pub origination_account_id: Option<String>,
    pub type_: BankTransferType,
    pub user: BankTransferUser,
}
pub struct BankTransferCreateRequired<'a> {
    pub access_token: &'a str,
    pub account_id: &'a str,
    pub amount: &'a str,
    pub description: &'a str,
    pub idempotency_key: &'a str,
    pub iso_currency_code: &'a str,
    pub network: BankTransferNetwork,
    pub type_: BankTransferType,
    pub user: BankTransferUser,
}
impl FluentRequest<'_, BankTransferCreateRequest> {
    ///Set the value of the ach_class field.
    pub fn ach_class(mut self, ach_class: AchClass) -> Self {
        self.params.ach_class = Some(ach_class);
        self
    }
    ///Set the value of the custom_tag field.
    pub fn custom_tag(mut self, custom_tag: &str) -> Self {
        self.params.custom_tag = Some(custom_tag.to_owned());
        self
    }
    ///Set the value of the metadata field.
    pub fn metadata(mut self, metadata: BankTransferMetadata) -> Self {
        self.params.metadata = Some(metadata);
        self
    }
    ///Set the value of the origination_account_id field.
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.params.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BankTransferCreateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::BankTransferCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/bank_transfer/create";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = r.json(serde_json::json!({ "account_id" : self.params.account_id }));
            if let Some(ref unwrapped) = self.params.ach_class {
                r = r.json(serde_json::json!({ "ach_class" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "amount" : self.params.amount }));
            if let Some(ref unwrapped) = self.params.custom_tag {
                r = r.json(serde_json::json!({ "custom_tag" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "description" : self.params.description }));
            r = r
                .json(
                    serde_json::json!(
                        { "idempotency_key" : self.params.idempotency_key }
                    ),
                );
            r = r
                .json(
                    serde_json::json!(
                        { "iso_currency_code" : self.params.iso_currency_code }
                    ),
                );
            if let Some(ref unwrapped) = self.params.metadata {
                r = r.json(serde_json::json!({ "metadata" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "network" : self.params.network }));
            if let Some(ref unwrapped) = self.params.origination_account_id {
                r = r.json(serde_json::json!({ "origination_account_id" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "type" : self.params.type_ }));
            r = r.json(serde_json::json!({ "user" : self.params.user }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create a bank transfer

Use the `/bank_transfer/create` endpoint to initiate a new bank transfer.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transfercreate>.*/
    pub fn bank_transfer_create(
        &self,
        args: BankTransferCreateRequired,
    ) -> FluentRequest<'_, BankTransferCreateRequest> {
        FluentRequest {
            client: self,
            params: BankTransferCreateRequest {
                access_token: args.access_token.to_owned(),
                account_id: args.account_id.to_owned(),
                ach_class: None,
                amount: args.amount.to_owned(),
                custom_tag: None,
                description: args.description.to_owned(),
                idempotency_key: args.idempotency_key.to_owned(),
                iso_currency_code: args.iso_currency_code.to_owned(),
                metadata: None,
                network: args.network,
                origination_account_id: None,
                type_: args.type_,
                user: args.user,
            },
        }
    }
}
