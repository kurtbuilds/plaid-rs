use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transactions_user_insights_get`].

On request success, this will return a [`TransactionsUserInsightsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsUserInsightsGetRequest {
    pub client_user_id: String,
}
impl FluentRequest<'_, TransactionsUserInsightsGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransactionsUserInsightsGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransactionsUserInsightsGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beta/transactions/user_insights/v1/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!({ "client_user_id" : self.params.client_user_id }),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Obtain user insights based on transactions sent through /transactions/enrich

The `/beta/transactions/user_insights/v1/get` gets user insights for clients who have enriched data with `/transactions/enrich`.

The product is currently in beta.

See endpoint docs at <https://plaid.com/docs/api/products/enrich/#userinsightsget>.*/
    pub fn transactions_user_insights_get(
        &self,
        client_user_id: &str,
    ) -> FluentRequest<'_, TransactionsUserInsightsGetRequest> {
        FluentRequest {
            client: self,
            params: TransactionsUserInsightsGetRequest {
                client_user_id: client_user_id.to_owned(),
            },
        }
    }
}
