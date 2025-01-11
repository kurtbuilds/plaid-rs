use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::TransactionsRuleDetails;
/**You should use this struct via [`PlaidClient::transactions_rules_create`].

On request success, this will return a [`TransactionsRulesCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsRulesCreateRequest {
    pub access_token: String,
    pub personal_finance_category: String,
    pub rule_details: TransactionsRuleDetails,
}
impl FluentRequest<'_, TransactionsRulesCreateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransactionsRulesCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransactionsRulesCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beta/transactions/rules/v1/create";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = r
                .json(
                    serde_json::json!(
                        { "personal_finance_category" : self.params
                        .personal_finance_category }
                    ),
                );
            r = r.json(serde_json::json!({ "rule_details" : self.params.rule_details }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create transaction category rule

The `/transactions/rules/v1/create` endpoint creates transaction categorization rules.

Rules will be applied on the Item's transactions returned in `/transactions/get` response.

The product is currently in beta. To request access, contact transactions-feedback@plaid.com.*/
    pub fn transactions_rules_create(
        &self,
        access_token: &str,
        personal_finance_category: &str,
        rule_details: TransactionsRuleDetails,
    ) -> FluentRequest<'_, TransactionsRulesCreateRequest> {
        FluentRequest {
            client: self,
            params: TransactionsRulesCreateRequest {
                access_token: access_token.to_owned(),
                personal_finance_category: personal_finance_category.to_owned(),
                rule_details,
            },
        }
    }
}
