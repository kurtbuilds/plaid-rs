use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transfer_originator_get`].

On request success, this will return a [`TransferOriginatorGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferOriginatorGetRequest {
    pub originator_client_id: String,
}
impl FluentRequest<'_, TransferOriginatorGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferOriginatorGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransferOriginatorGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/originator/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "originator_client_id" : self.params.originator_client_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get status of an originator's onboarding

The `/transfer/originator/get` endpoint gets status updates for an originator's onboarding process. This information is also available via the Transfer page on the Plaid dashboard.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/platform-payments/#transferoriginatorget>.*/
    pub fn transfer_originator_get(
        &self,
        originator_client_id: &str,
    ) -> FluentRequest<'_, TransferOriginatorGetRequest> {
        FluentRequest {
            client: self,
            params: TransferOriginatorGetRequest {
                originator_client_id: originator_client_id.to_owned(),
            },
        }
    }
}
