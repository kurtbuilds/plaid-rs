use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::IdentityMatchUser;
/**You should use this struct via [`PlaidClient::processor_identity_match`].

On request success, this will return a [`ProcessorIdentityMatchResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorIdentityMatchRequest {
    pub processor_token: String,
    pub user: Option<IdentityMatchUser>,
}
impl FluentRequest<'_, ProcessorIdentityMatchRequest> {
    ///Set the value of the user field.
    pub fn user(mut self, user: IdentityMatchUser) -> Self {
        self.params.user = Some(user);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ProcessorIdentityMatchRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::ProcessorIdentityMatchResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/identity/match";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "processor_token" : self.params.processor_token }
                    ),
                );
            if let Some(ref unwrapped) = self.params.user {
                r = r.json(serde_json::json!({ "user" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve identity match score

The `/processor/identity/match` endpoint generates a match score, which indicates how well the provided identity data matches the identity information on file with the account holder's financial institution.

Fields within the `balances` object will always be null when retrieved by `/identity/match`. Instead, use the free `/accounts/get` endpoint to request balance cached data, or `/accounts/balance/get` for real-time data.

See endpoint docs at <https://plaid.com/docs/api/processor-partners/#processoridentitymatch>.*/
    pub fn processor_identity_match(
        &self,
        processor_token: &str,
    ) -> FluentRequest<'_, ProcessorIdentityMatchRequest> {
        FluentRequest {
            client: self,
            params: ProcessorIdentityMatchRequest {
                processor_token: processor_token.to_owned(),
                user: None,
            },
        }
    }
}
