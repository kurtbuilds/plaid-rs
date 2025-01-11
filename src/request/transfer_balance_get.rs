use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::TransferBalanceType;
/**You should use this struct via [`PlaidClient::transfer_balance_get`].

On request success, this will return a [`TransferBalanceGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferBalanceGetRequest {
    pub originator_client_id: Option<String>,
    pub type_: Option<TransferBalanceType>,
}
impl FluentRequest<'_, TransferBalanceGetRequest> {
    ///Set the value of the originator_client_id field.
    pub fn originator_client_id(mut self, originator_client_id: &str) -> Self {
        self.params.originator_client_id = Some(originator_client_id.to_owned());
        self
    }
    ///Set the value of the type_ field.
    pub fn type_(mut self, type_: TransferBalanceType) -> Self {
        self.params.type_ = Some(type_);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferBalanceGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::TransferBalanceGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/balance/get";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.originator_client_id {
                r = r.json(serde_json::json!({ "originator_client_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.type_ {
                r = r.json(serde_json::json!({ "type" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**(Deprecated) Retrieve a balance held with Plaid

(Deprecated) Use the `/transfer/balance/get` endpoint to view a balance held with Plaid.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/balance/#transferbalanceget>.*/
    pub fn transfer_balance_get(&self) -> FluentRequest<'_, TransferBalanceGetRequest> {
        FluentRequest {
            client: self,
            params: TransferBalanceGetRequest {
                originator_client_id: None,
                type_: None,
            },
        }
    }
}
