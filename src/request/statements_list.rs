use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::statements_list`].

On request success, this will return a [`StatementsListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatementsListRequest {
    pub access_token: String,
}
impl FluentRequest<'_, StatementsListRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, StatementsListRequest> {
    type Output = httpclient::InMemoryResult<crate::model::StatementsListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/statements/list";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve a list of all statements associated with an item.

The `/statements/list` endpoint retrieves a list of all statements associated with an item.

See endpoint docs at <https://plaid.com/docs/api/products/statements#statementslist>.*/
    pub fn statements_list(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, StatementsListRequest> {
        FluentRequest {
            client: self,
            params: StatementsListRequest {
                access_token: access_token.to_owned(),
            },
        }
    }
}
