use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::processor_token_create`].

On request success, this will return a [`ProcessorTokenCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorTokenCreateRequest {
    pub access_token: String,
    pub account_id: String,
    pub processor: String,
}
impl FluentRequest<'_, ProcessorTokenCreateRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ProcessorTokenCreateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::ProcessorTokenCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/token/create";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = r.json(serde_json::json!({ "account_id" : self.params.account_id }));
            r = r.json(serde_json::json!({ "processor" : self.params.processor }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create processor token

Used to create a token suitable for sending to one of Plaid's partners to enable integrations. Note that Stripe partnerships use bank account tokens instead; see `/processor/stripe/bank_account_token/create` for creating tokens for use with Stripe integrations. If using multiple processors, multiple different processor tokens can be created for a single access token. Once created, a processor token for a given Item cannot be modified or updated. To revoke the processor's access, the entire Item must be deleted by calling `/item/remove`.

See endpoint docs at <https://plaid.com/docs/api/processors/#processortokencreate>.*/
    pub fn processor_token_create(
        &self,
        access_token: &str,
        account_id: &str,
        processor: &str,
    ) -> FluentRequest<'_, ProcessorTokenCreateRequest> {
        FluentRequest {
            client: self,
            params: ProcessorTokenCreateRequest {
                access_token: access_token.to_owned(),
                account_id: account_id.to_owned(),
                processor: processor.to_owned(),
            },
        }
    }
}
