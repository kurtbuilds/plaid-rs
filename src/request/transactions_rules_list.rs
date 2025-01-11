use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transactions_rules_list`].

On request success, this will return a [`TransactionsRulesListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsRulesListRequest {
    pub access_token: String,
}
impl FluentRequest<'_, TransactionsRulesListRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransactionsRulesListRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransactionsRulesListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beta/transactions/rules/v1/list";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Return a list of rules created for the Item associated with the access token.

The `/transactions/rules/v1/list` returns a list of transaction rules created for the Item associated with the access token.*/
    pub fn transactions_rules_list(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, TransactionsRulesListRequest> {
        FluentRequest {
            client: self,
            params: TransactionsRulesListRequest {
                access_token: access_token.to_owned(),
            },
        }
    }
}
