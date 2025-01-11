use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::link_delivery_get`].

On request success, this will return a [`LinkDeliveryGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkDeliveryGetRequest {
    pub link_delivery_session_id: String,
}
impl FluentRequest<'_, LinkDeliveryGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, LinkDeliveryGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::LinkDeliveryGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/link_delivery/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "link_delivery_session_id" : self.params
                        .link_delivery_session_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get Hosted Link session

Use the `/link_delivery/get` endpoint to get the status of a Hosted Link session.

See endpoint docs at <https://plaid.com/docs/assets/waitlist/hosted-link/>.*/
    pub fn link_delivery_get(
        &self,
        link_delivery_session_id: &str,
    ) -> FluentRequest<'_, LinkDeliveryGetRequest> {
        FluentRequest {
            client: self,
            params: LinkDeliveryGetRequest {
                link_delivery_session_id: link_delivery_session_id.to_owned(),
            },
        }
    }
}
