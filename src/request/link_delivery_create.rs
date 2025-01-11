use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::LinkDeliveryOptions;
/**You should use this struct via [`PlaidClient::link_delivery_create`].

On request success, this will return a [`LinkDeliveryCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkDeliveryCreateRequest {
    pub link_token: String,
    pub options: Option<LinkDeliveryOptions>,
}
impl FluentRequest<'_, LinkDeliveryCreateRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: LinkDeliveryOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, LinkDeliveryCreateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::LinkDeliveryCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/link_delivery/create";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "link_token" : self.params.link_token }));
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
    /**Create Hosted Link session

Use the `/link_delivery/create` endpoint to create a Hosted Link session.

See endpoint docs at <https://plaid.com/docs/assets/waitlist/hosted-link/>.*/
    pub fn link_delivery_create(
        &self,
        link_token: &str,
    ) -> FluentRequest<'_, LinkDeliveryCreateRequest> {
        FluentRequest {
            client: self,
            params: LinkDeliveryCreateRequest {
                link_token: link_token.to_owned(),
                options: None,
            },
        }
    }
}
