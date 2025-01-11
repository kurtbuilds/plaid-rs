use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::statements_refresh`].

On request success, this will return a [`StatementsRefreshResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatementsRefreshRequest {
    pub access_token: String,
    pub end_date: chrono::NaiveDate,
    pub start_date: chrono::NaiveDate,
}
impl FluentRequest<'_, StatementsRefreshRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, StatementsRefreshRequest> {
    type Output = httpclient::InMemoryResult<crate::model::StatementsRefreshResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/statements/refresh";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = r.json(serde_json::json!({ "end_date" : self.params.end_date }));
            r = r.json(serde_json::json!({ "start_date" : self.params.start_date }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Refresh statements data.

`/statements/refresh` initiates an on-demand extraction to fetch the statements for the provided dates.

See endpoint docs at <https://plaid.com/docs/api/products/statements#statementsrefresh>.*/
    pub fn statements_refresh(
        &self,
        access_token: &str,
        end_date: chrono::NaiveDate,
        start_date: chrono::NaiveDate,
    ) -> FluentRequest<'_, StatementsRefreshRequest> {
        FluentRequest {
            client: self,
            params: StatementsRefreshRequest {
                access_token: access_token.to_owned(),
                end_date,
                start_date,
            },
        }
    }
}
