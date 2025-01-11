use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transactions_rules_remove`].

On request success, this will return a [`TransactionsRulesRemoveResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsRulesRemoveRequest {
    pub access_token: String,
    pub rule_id: String,
}
impl FluentRequest<'_, TransactionsRulesRemoveRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransactionsRulesRemoveRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransactionsRulesRemoveResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beta/transactions/rules/v1/remove";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = r.json(serde_json::json!({ "rule_id" : self.params.rule_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Remove transaction rule

The `/transactions/rules/v1/remove` endpoint is used to remove a transaction rule.*/
    pub fn transactions_rules_remove(
        &self,
        access_token: &str,
        rule_id: &str,
    ) -> FluentRequest<'_, TransactionsRulesRemoveRequest> {
        FluentRequest {
            client: self,
            params: TransactionsRulesRemoveRequest {
                access_token: access_token.to_owned(),
                rule_id: rule_id.to_owned(),
            },
        }
    }
}
