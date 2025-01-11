use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::get_recipients`].

On request success, this will return a [`GetRecipientsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRecipientsRequest {}
impl FluentRequest<'_, GetRecipientsRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetRecipientsRequest> {
    type Output = httpclient::InMemoryResult<crate::model::GetRecipientsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/fdx/recipients";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get Recipients

Returns a list of Recipients*/
    pub fn get_recipients(&self) -> FluentRequest<'_, GetRecipientsRequest> {
        FluentRequest {
            client: self,
            params: GetRecipientsRequest {},
        }
    }
}
