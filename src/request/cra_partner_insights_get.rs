use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::cra_partner_insights_get`].

On request success, this will return a [`CraPartnerInsightsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraPartnerInsightsGetRequest {
    pub user_token: String,
}
impl FluentRequest<'_, CraPartnerInsightsGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CraPartnerInsightsGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::CraPartnerInsightsGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/cra/partner_insights/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "user_token" : self.params.user_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve cash flow insights from the bank accounts used for income verification

`/cra/partner_insights/get` returns cash flow insights for a specified user.

See endpoint docs at <https://plaid.com/docs/api/products/income/#crapartner_insightsget>.*/
    pub fn cra_partner_insights_get(
        &self,
        user_token: &str,
    ) -> FluentRequest<'_, CraPartnerInsightsGetRequest> {
        FluentRequest {
            client: self,
            params: CraPartnerInsightsGetRequest {
                user_token: user_token.to_owned(),
            },
        }
    }
}
