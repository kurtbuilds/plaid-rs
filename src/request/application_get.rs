use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::application_get`].

On request success, this will return a [`ApplicationGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationGetRequest {
    pub application_id: String,
}
impl FluentRequest<'_, ApplicationGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ApplicationGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::ApplicationGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/application/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!({ "application_id" : self.params.application_id }),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve information about a Plaid application

Allows financial institutions to retrieve information about Plaid clients for the purpose of building control-tower experiences*/
    pub fn application_get(
        &self,
        application_id: &str,
    ) -> FluentRequest<'_, ApplicationGetRequest> {
        FluentRequest {
            client: self,
            params: ApplicationGetRequest {
                application_id: application_id.to_owned(),
            },
        }
    }
}
