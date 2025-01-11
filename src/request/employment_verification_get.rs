use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::employment_verification_get`].

On request success, this will return a [`EmploymentVerificationGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmploymentVerificationGetRequest {
    pub access_token: String,
}
impl FluentRequest<'_, EmploymentVerificationGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, EmploymentVerificationGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::EmploymentVerificationGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/employment/verification/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**(Deprecated) Retrieve a summary of an individual's employment information

`/employment/verification/get` returns a list of employments through a user payroll that was verified by an end user.

This endpoint has been deprecated; new integrations should use `/credit/employment/get` instead.

See endpoint docs at <https://plaid.com/docs/api/products/income/#employmentverificationget>.*/
    pub fn employment_verification_get(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, EmploymentVerificationGetRequest> {
        FluentRequest {
            client: self,
            params: EmploymentVerificationGetRequest {
                access_token: access_token.to_owned(),
            },
        }
    }
}
