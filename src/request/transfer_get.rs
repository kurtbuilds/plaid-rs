use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transfer_get`].

On request success, this will return a [`TransferGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferGetRequest {
    pub authorization_id: Option<String>,
    pub originator_client_id: Option<String>,
    pub transfer_id: Option<String>,
}
impl FluentRequest<'_, TransferGetRequest> {
    ///Set the value of the authorization_id field.
    pub fn authorization_id(mut self, authorization_id: &str) -> Self {
        self.params.authorization_id = Some(authorization_id.to_owned());
        self
    }
    ///Set the value of the originator_client_id field.
    pub fn originator_client_id(mut self, originator_client_id: &str) -> Self {
        self.params.originator_client_id = Some(originator_client_id.to_owned());
        self
    }
    ///Set the value of the transfer_id field.
    pub fn transfer_id(mut self, transfer_id: &str) -> Self {
        self.params.transfer_id = Some(transfer_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::TransferGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/get";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.authorization_id {
                r = r.json(serde_json::json!({ "authorization_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.originator_client_id {
                r = r.json(serde_json::json!({ "originator_client_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.transfer_id {
                r = r.json(serde_json::json!({ "transfer_id" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve a transfer

The `/transfer/get` endpoint fetches information about the transfer corresponding to the given `transfer_id` or `authorization_id`. One of `transfer_id` or `authorization_id` must be populated but not both.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/reading-transfers/#transferget>.*/
    pub fn transfer_get(&self) -> FluentRequest<'_, TransferGetRequest> {
        FluentRequest {
            client: self,
            params: TransferGetRequest {
                authorization_id: None,
                originator_client_id: None,
                transfer_id: None,
            },
        }
    }
}
