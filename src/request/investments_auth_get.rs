use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::InvestmentsAuthGetRequestOptions;
/**You should use this struct via [`PlaidClient::investments_auth_get`].

On request success, this will return a [`InvestmentsAuthGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentsAuthGetRequest {
    pub access_token: String,
    pub options: Option<InvestmentsAuthGetRequestOptions>,
}
impl FluentRequest<'_, InvestmentsAuthGetRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: InvestmentsAuthGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, InvestmentsAuthGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::InvestmentsAuthGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/investments/auth/get";
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
    /**Get data needed to authorize an investments transfer

The `/investments/auth/get` endpoint allows developers to receive user-authorized data to facilitate the transfer of holdings

See endpoint docs at <https://plaid.com/docs/api/products/investments-move/#investmentsauthget>.*/
    pub fn investments_auth_get(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, InvestmentsAuthGetRequest> {
        FluentRequest {
            client: self,
            params: InvestmentsAuthGetRequest {
                access_token: access_token.to_owned(),
                options: None,
            },
        }
    }
}
