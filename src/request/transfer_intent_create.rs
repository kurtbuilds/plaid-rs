use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{
    ACHClass, TransferMetadata, TransferIntentCreateMode, TransferIntentCreateNetwork,
    TransferUserInRequest,
};
/**You should use this struct via [`PlaidClient::transfer_intent_create`].

On request success, this will return a [`TransferIntentCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferIntentCreateRequest {
    pub account_id: Option<String>,
    pub ach_class: Option<AchClass>,
    pub amount: String,
    pub description: String,
    pub funding_account_id: Option<String>,
    pub iso_currency_code: Option<String>,
    pub metadata: Option<TransferMetadata>,
    pub mode: TransferIntentCreateMode,
    pub network: Option<TransferIntentCreateNetwork>,
    pub origination_account_id: Option<String>,
    pub require_guarantee: Option<bool>,
    pub user: TransferUserInRequest,
}
pub struct TransferIntentCreateRequired<'a> {
    pub amount: &'a str,
    pub description: &'a str,
    pub mode: TransferIntentCreateMode,
    pub user: TransferUserInRequest,
}
impl FluentRequest<'_, TransferIntentCreateRequest> {
    ///Set the value of the account_id field.
    pub fn account_id(mut self, account_id: &str) -> Self {
        self.params.account_id = Some(account_id.to_owned());
        self
    }
    ///Set the value of the ach_class field.
    pub fn ach_class(mut self, ach_class: AchClass) -> Self {
        self.params.ach_class = Some(ach_class);
        self
    }
    ///Set the value of the funding_account_id field.
    pub fn funding_account_id(mut self, funding_account_id: &str) -> Self {
        self.params.funding_account_id = Some(funding_account_id.to_owned());
        self
    }
    ///Set the value of the iso_currency_code field.
    pub fn iso_currency_code(mut self, iso_currency_code: &str) -> Self {
        self.params.iso_currency_code = Some(iso_currency_code.to_owned());
        self
    }
    ///Set the value of the metadata field.
    pub fn metadata(mut self, metadata: TransferMetadata) -> Self {
        self.params.metadata = Some(metadata);
        self
    }
    ///Set the value of the network field.
    pub fn network(mut self, network: TransferIntentCreateNetwork) -> Self {
        self.params.network = Some(network);
        self
    }
    ///Set the value of the origination_account_id field.
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.params.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
    ///Set the value of the require_guarantee field.
    pub fn require_guarantee(mut self, require_guarantee: bool) -> Self {
        self.params.require_guarantee = Some(require_guarantee);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferIntentCreateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::TransferIntentCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/intent/create";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.account_id {
                r = r.json(serde_json::json!({ "account_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.ach_class {
                r = r.json(serde_json::json!({ "ach_class" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "amount" : self.params.amount }));
            r = r.json(serde_json::json!({ "description" : self.params.description }));
            if let Some(ref unwrapped) = self.params.funding_account_id {
                r = r.json(serde_json::json!({ "funding_account_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.iso_currency_code {
                r = r.json(serde_json::json!({ "iso_currency_code" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.metadata {
                r = r.json(serde_json::json!({ "metadata" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "mode" : self.params.mode }));
            if let Some(ref unwrapped) = self.params.network {
                r = r.json(serde_json::json!({ "network" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.origination_account_id {
                r = r.json(serde_json::json!({ "origination_account_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.require_guarantee {
                r = r.json(serde_json::json!({ "require_guarantee" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "user" : self.params.user }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create a transfer intent object to invoke the Transfer UI

Use the `/transfer/intent/create` endpoint to generate a transfer intent object and invoke the Transfer UI.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/account-linking/#transferintentcreate>.*/
    pub fn transfer_intent_create(
        &self,
        args: TransferIntentCreateRequired,
    ) -> FluentRequest<'_, TransferIntentCreateRequest> {
        FluentRequest {
            client: self,
            params: TransferIntentCreateRequest {
                account_id: None,
                ach_class: None,
                amount: args.amount.to_owned(),
                description: args.description.to_owned(),
                funding_account_id: None,
                iso_currency_code: None,
                metadata: None,
                mode: args.mode,
                network: None,
                origination_account_id: None,
                require_guarantee: None,
                user: args.user,
            },
        }
    }
}
