use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_migrate_account`].

On request success, this will return a [`TransferMigrateAccountResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferMigrateAccountRequest {
    pub account_number: String,
    pub account_type: String,
    pub routing_number: String,
    pub wire_routing_number: Option<String>,
}
impl TransferMigrateAccountRequest {}
impl FluentRequest<'_, TransferMigrateAccountRequest> {
    pub fn wire_routing_number(mut self, wire_routing_number: &str) -> Self {
        self.params.wire_routing_number = Some(wire_routing_number.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferMigrateAccountRequest> {
    type Output = httpclient::InMemoryResult<TransferMigrateAccountResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/migrate_account";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "account_number" : self.params.account_number }));
            r = r.json(json!({ "account_type" : self.params.account_type }));
            r = r.json(json!({ "routing_number" : self.params.routing_number }));
            if let Some(ref unwrapped) = self.params.wire_routing_number {
                r = r.json(json!({ "wire_routing_number" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}