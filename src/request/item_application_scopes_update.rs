use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{ScopesContext, Scopes};
/**You should use this struct via [`PlaidClient::item_application_scopes_update`].

On request success, this will return a [`ItemApplicationScopesUpdateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemApplicationScopesUpdateRequest {
    pub access_token: String,
    pub application_id: String,
    pub context: ScopesContext,
    pub scopes: Scopes,
    pub state: Option<String>,
}
pub struct ItemApplicationScopesUpdateRequired<'a> {
    pub access_token: &'a str,
    pub application_id: &'a str,
    pub context: ScopesContext,
    pub scopes: Scopes,
}
impl FluentRequest<'_, ItemApplicationScopesUpdateRequest> {
    ///Set the value of the state field.
    pub fn state(mut self, state: &str) -> Self {
        self.params.state = Some(state.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ItemApplicationScopesUpdateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::ItemApplicationScopesUpdateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/item/application/scopes/update";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = r
                .json(
                    serde_json::json!({ "application_id" : self.params.application_id }),
                );
            r = r.json(serde_json::json!({ "context" : self.params.context }));
            r = r.json(serde_json::json!({ "scopes" : self.params.scopes }));
            if let Some(ref unwrapped) = self.params.state {
                r = r.json(serde_json::json!({ "state" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Update the scopes of access for a particular application

Enable consumers to update product access on selected accounts for an application.*/
    pub fn item_application_scopes_update(
        &self,
        args: ItemApplicationScopesUpdateRequired,
    ) -> FluentRequest<'_, ItemApplicationScopesUpdateRequest> {
        FluentRequest {
            client: self,
            params: ItemApplicationScopesUpdateRequest {
                access_token: args.access_token.to_owned(),
                application_id: args.application_id.to_owned(),
                context: args.context,
                scopes: args.scopes,
                state: None,
            },
        }
    }
}
