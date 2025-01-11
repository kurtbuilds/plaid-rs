use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::webhook_verification_key_get`].

On request success, this will return a [`WebhookVerificationKeyGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookVerificationKeyGetRequest {
    pub key_id: String,
}
impl FluentRequest<'_, WebhookVerificationKeyGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WebhookVerificationKeyGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::WebhookVerificationKeyGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/webhook_verification_key/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "key_id" : self.params.key_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get webhook verification key

Plaid signs all outgoing webhooks and provides JSON Web Tokens (JWTs) so that you can verify the authenticity of any incoming webhooks to your application. A message signature is included in the `Plaid-Verification` header.

The `/webhook_verification_key/get` endpoint provides a JSON Web Key (JWK) that can be used to verify a JWT.

See endpoint docs at <https://plaid.com/docs/api/webhooks/webhook-verification/#get-webhook-verification-key>.*/
    pub fn webhook_verification_key_get(
        &self,
        key_id: &str,
    ) -> FluentRequest<'_, WebhookVerificationKeyGetRequest> {
        FluentRequest {
            client: self,
            params: WebhookVerificationKeyGetRequest {
                key_id: key_id.to_owned(),
            },
        }
    }
}
