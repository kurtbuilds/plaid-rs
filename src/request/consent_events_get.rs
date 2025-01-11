use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::consent_events_get`].

On request success, this will return a [`ConsentEventsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentEventsGetRequest {
    pub access_token: String,
}
impl FluentRequest<'_, ConsentEventsGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ConsentEventsGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::ConsentEventsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/consent/events/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**List a historical log of item consent events

See endpoint docs at <https://plaid.com/docs/api/consent/#consenteventsget>.*/
    pub fn consent_events_get(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, ConsentEventsGetRequest> {
        FluentRequest {
            client: self,
            params: ConsentEventsGetRequest {
                access_token: access_token.to_owned(),
            },
        }
    }
}
