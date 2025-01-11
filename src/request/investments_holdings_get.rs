use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::InvestmentHoldingsGetRequestOptions;
/**You should use this struct via [`PlaidClient::investments_holdings_get`].

On request success, this will return a [`InvestmentsHoldingsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentsHoldingsGetRequest {
    pub access_token: String,
    pub options: Option<InvestmentHoldingsGetRequestOptions>,
}
impl FluentRequest<'_, InvestmentsHoldingsGetRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: InvestmentHoldingsGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, InvestmentsHoldingsGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::InvestmentsHoldingsGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/investments/holdings/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get Investment holdings

The `/investments/holdings/get` endpoint allows developers to receive user-authorized stock position data for `investment`-type accounts.

See endpoint docs at <https://plaid.com/docs/api/products/investments/#investmentsholdingsget>.*/
    pub fn investments_holdings_get(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, InvestmentsHoldingsGetRequest> {
        FluentRequest {
            client: self,
            params: InvestmentsHoldingsGetRequest {
                access_token: access_token.to_owned(),
                options: None,
            },
        }
    }
}
