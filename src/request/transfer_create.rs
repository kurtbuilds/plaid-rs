use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{
    AchClass, TransferMetadata, TransferNetwork, TransferType,
    TransferUserInRequestDeprecated,
};
/**You should use this struct via [`PlaidClient::transfer_create`].

On request success, this will return a [`TransferCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferCreateRequest {
    pub access_token: String,
    pub account_id: String,
    pub ach_class: Option<AchClass>,
    pub amount: Option<String>,
    pub authorization_id: String,
    pub description: String,
    pub facilitator_fee: Option<String>,
    pub idempotency_key: Option<String>,
    pub iso_currency_code: Option<String>,
    pub metadata: Option<TransferMetadata>,
    pub network: Option<TransferNetwork>,
    pub origination_account_id: Option<String>,
    pub test_clock_id: Option<String>,
    pub type_: Option<TransferType>,
    pub user: Option<TransferUserInRequestDeprecated>,
}
pub struct TransferCreateRequired<'a> {
    pub access_token: &'a str,
    pub account_id: &'a str,
    pub authorization_id: &'a str,
    pub description: &'a str,
}
impl FluentRequest<'_, TransferCreateRequest> {
    ///Set the value of the ach_class field.
    pub fn ach_class(mut self, ach_class: AchClass) -> Self {
        self.params.ach_class = Some(ach_class);
        self
    }
    ///Set the value of the amount field.
    pub fn amount(mut self, amount: &str) -> Self {
        self.params.amount = Some(amount.to_owned());
        self
    }
    ///Set the value of the facilitator_fee field.
    pub fn facilitator_fee(mut self, facilitator_fee: &str) -> Self {
        self.params.facilitator_fee = Some(facilitator_fee.to_owned());
        self
    }
    ///Set the value of the idempotency_key field.
    pub fn idempotency_key(mut self, idempotency_key: &str) -> Self {
        self.params.idempotency_key = Some(idempotency_key.to_owned());
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
    pub fn network(mut self, network: TransferNetwork) -> Self {
        self.params.network = Some(network);
        self
    }
    ///Set the value of the origination_account_id field.
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.params.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
    ///Set the value of the test_clock_id field.
    pub fn test_clock_id(mut self, test_clock_id: &str) -> Self {
        self.params.test_clock_id = Some(test_clock_id.to_owned());
        self
    }
    ///Set the value of the type_ field.
    pub fn type_(mut self, type_: TransferType) -> Self {
        self.params.type_ = Some(type_);
        self
    }
    ///Set the value of the user field.
    pub fn user(mut self, user: TransferUserInRequestDeprecated) -> Self {
        self.params.user = Some(user);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferCreateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::TransferCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/create";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = r.json(serde_json::json!({ "account_id" : self.params.account_id }));
            if let Some(ref unwrapped) = self.params.ach_class {
                r = r.json(serde_json::json!({ "ach_class" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.amount {
                r = r.json(serde_json::json!({ "amount" : unwrapped }));
            }
            r = r
                .json(
                    serde_json::json!(
                        { "authorization_id" : self.params.authorization_id }
                    ),
                );
            r = r.json(serde_json::json!({ "description" : self.params.description }));
            if let Some(ref unwrapped) = self.params.facilitator_fee {
                r = r.json(serde_json::json!({ "facilitator_fee" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.idempotency_key {
                r = r.json(serde_json::json!({ "idempotency_key" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.iso_currency_code {
                r = r.json(serde_json::json!({ "iso_currency_code" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.metadata {
                r = r.json(serde_json::json!({ "metadata" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.network {
                r = r.json(serde_json::json!({ "network" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.origination_account_id {
                r = r.json(serde_json::json!({ "origination_account_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.test_clock_id {
                r = r.json(serde_json::json!({ "test_clock_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.type_ {
                r = r.json(serde_json::json!({ "type" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.user {
                r = r.json(serde_json::json!({ "user" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create a transfer

Use the `/transfer/create` endpoint to initiate a new transfer. This endpoint is retryable and idempotent; if a transfer with the provided `transfer_id` has already been created, it will return the transfer details without creating a new transfer. A transfer may still be created if a 500 error is returned; to detect this scenario, use [Transfer events](https://plaid.com/docs/transfer/reconciling-transfers/).

See endpoint docs at <https://plaid.com/docs/api/products/transfer/initiating-transfers/#transfercreate>.*/
    pub fn transfer_create(
        &self,
        args: TransferCreateRequired,
    ) -> FluentRequest<'_, TransferCreateRequest> {
        FluentRequest {
            client: self,
            params: TransferCreateRequest {
                access_token: args.access_token.to_owned(),
                account_id: args.account_id.to_owned(),
                ach_class: None,
                amount: None,
                authorization_id: args.authorization_id.to_owned(),
                description: args.description.to_owned(),
                facilitator_fee: None,
                idempotency_key: None,
                iso_currency_code: None,
                metadata: None,
                network: None,
                origination_account_id: None,
                test_clock_id: None,
                type_: None,
                user: None,
            },
        }
    }
}
