use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::LiabilitiesGetRequestOptions;
/**You should use this struct via [`PlaidClient::liabilities_get`].

On request success, this will return a [`LiabilitiesGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiabilitiesGetRequest {
    pub access_token: String,
    pub options: Option<LiabilitiesGetRequestOptions>,
}
impl FluentRequest<'_, LiabilitiesGetRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: LiabilitiesGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, LiabilitiesGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::LiabilitiesGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/liabilities/get";
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
    /**Retrieve Liabilities data

The `/liabilities/get` endpoint returns various details about an Item with loan or credit accounts. Liabilities data is available primarily for US financial institutions, with some limited coverage of Canadian institutions. Currently supported account types are account type `credit` with account subtype `credit card` or `paypal`, and account type `loan` with account subtype `student` or `mortgage`. To limit accounts listed in Link to types and subtypes supported by Liabilities, you can use the `account_filters` parameter when [creating a Link token](https://plaid.com/docs/api/link/#linktokencreate).

The types of information returned by Liabilities can include balances and due dates, loan terms, and account details such as original loan amount and guarantor. Data is refreshed approximately once per day; the latest data can be retrieved by calling `/liabilities/get`.

See endpoint docs at <https://plaid.com/docs/api/products/liabilities/#liabilitiesget>.*/
    pub fn liabilities_get(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, LiabilitiesGetRequest> {
        FluentRequest {
            client: self,
            params: LiabilitiesGetRequest {
                access_token: access_token.to_owned(),
                options: None,
            },
        }
    }
}
