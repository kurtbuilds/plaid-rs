use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transfer_originator_list`].

On request success, this will return a [`TransferOriginatorListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferOriginatorListRequest {
    pub count: Option<i64>,
    pub offset: Option<i64>,
}
impl FluentRequest<'_, TransferOriginatorListRequest> {
    ///Set the value of the count field.
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
    ///Set the value of the offset field.
    pub fn offset(mut self, offset: i64) -> Self {
        self.params.offset = Some(offset);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferOriginatorListRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransferOriginatorListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/originator/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.count {
                r = r.json(serde_json::json!({ "count" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.offset {
                r = r.json(serde_json::json!({ "offset" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get status of all originators' onboarding

The `/transfer/originator/list` endpoint gets status updates for all of your originators' onboarding. This information is also available via the Plaid dashboard.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/platform-payments/#transferoriginatorlist>.*/
    pub fn transfer_originator_list(
        &self,
    ) -> FluentRequest<'_, TransferOriginatorListRequest> {
        FluentRequest {
            client: self,
            params: TransferOriginatorListRequest {
                count: None,
                offset: None,
            },
        }
    }
}
