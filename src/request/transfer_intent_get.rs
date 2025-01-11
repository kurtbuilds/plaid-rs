use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transfer_intent_get`].

On request success, this will return a [`TransferIntentGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferIntentGetRequest {
    pub transfer_intent_id: String,
}
impl FluentRequest<'_, TransferIntentGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferIntentGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::TransferIntentGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/intent/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "transfer_intent_id" : self.params.transfer_intent_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve more information about a transfer intent

Use the `/transfer/intent/get` endpoint to retrieve more information about a transfer intent.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/account-linking/#transferintentget>.*/
    pub fn transfer_intent_get(
        &self,
        transfer_intent_id: &str,
    ) -> FluentRequest<'_, TransferIntentGetRequest> {
        FluentRequest {
            client: self,
            params: TransferIntentGetRequest {
                transfer_intent_id: transfer_intent_id.to_owned(),
            },
        }
    }
}
