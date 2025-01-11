use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::identity_refresh`].

On request success, this will return a [`IdentityRefreshResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityRefreshRequest {
    pub access_token: String,
}
impl FluentRequest<'_, IdentityRefreshRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, IdentityRefreshRequest> {
    type Output = httpclient::InMemoryResult<crate::model::IdentityRefreshResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/identity/refresh";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Refresh identity data

`/identity/refresh` is an optional endpoint for users of the Identity product. It initiates an on-demand extraction to fetch the most up to date Identity information from the Financial Institution. This on-demand extraction takes place in addition to the periodic extractions that automatically occur for any Identity-enabled Item. If changes to Identity are discovered after calling `/identity/refresh`, Plaid will fire a webhook [`DEFAULT_UPDATE`](https://plaid.com/docs/api/products/identity/#default_update).

As this endpoint triggers a synchronous request for fresh data, latency may be higher than for other Plaid endpoints (typically less than 10 seconds, but occasionally up to 30 seconds or more); if you encounter errors, you may find it necessary to adjust your timeout period when making requests.

`/identity/refresh` is offered as an add-on to Identity and has a separate [fee model](/docs/account/billing/#per-request-flat-fee). To request access to this endpoint, submit a [product access request](https://dashboard.plaid.com/team/products) or contact your Plaid account manager.

See endpoint docs at <https://plaid.com/docs/api/products/identity/#identityrefresh>.*/
    pub fn identity_refresh(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, IdentityRefreshRequest> {
        FluentRequest {
            client: self,
            params: IdentityRefreshRequest {
                access_token: access_token.to_owned(),
            },
        }
    }
}
