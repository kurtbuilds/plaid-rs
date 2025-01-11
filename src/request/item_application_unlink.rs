use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::item_application_unlink`].

On request success, this will return a [`ItemApplicationUnlinkResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemApplicationUnlinkRequest {
    pub access_token: String,
    pub application_id: String,
}
impl FluentRequest<'_, ItemApplicationUnlinkRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ItemApplicationUnlinkRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::ItemApplicationUnlinkResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/item/application/unlink";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
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
    /**Unlink a user’s connected application

Unlink a user’s connected application. On an unlink request, Plaid will immediately revoke the Application’s access to the User’s data.  The User will have to redo the OAuth authentication process in order to restore functionality.

This endpoint only removes ongoing data access permissions, therefore the User will need to reach out to the Application itself in order to disable and delete their account and delete any data that the Application already received (if the Application does not do so by default).

This endpoint should be called in real time as the User is unlinking an Application, and should not be batched in order to ensure that the change is reflected as soon as possible.

See endpoint docs at <https://plaid.com/docsnone>.*/
    pub fn item_application_unlink(
        &self,
        access_token: &str,
        application_id: &str,
    ) -> FluentRequest<'_, ItemApplicationUnlinkRequest> {
        FluentRequest {
            client: self,
            params: ItemApplicationUnlinkRequest {
                access_token: access_token.to_owned(),
                application_id: application_id.to_owned(),
            },
        }
    }
}
