use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::get_recipient`].

On request success, this will return a [`GetRecipientResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRecipientRequest {
    pub oauth_state_id: Option<String>,
    pub recipient_id: String,
}
impl FluentRequest<'_, GetRecipientRequest> {
    ///Set the value of the oauth_state_id field.
    pub fn oauth_state_id(mut self, oauth_state_id: &str) -> Self {
        self.params.oauth_state_id = Some(oauth_state_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetRecipientRequest> {
    type Output = httpclient::InMemoryResult<crate::model::GetRecipientResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/fdx/recipient/{recipient_id}", recipient_id = self.params.recipient_id
            );
            let mut r = self.client.client.get(url);
            if let Some(ref unwrapped) = self.params.oauth_state_id {
                r = r.header("OAUTH-STATE-ID", &unwrapped.to_string());
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get Recipient

Get a specific recipient*/
    pub fn get_recipient(
        &self,
        recipient_id: &str,
    ) -> FluentRequest<'_, GetRecipientRequest> {
        FluentRequest {
            client: self,
            params: GetRecipientRequest {
                oauth_state_id: None,
                recipient_id: recipient_id.to_owned(),
            },
        }
    }
}
