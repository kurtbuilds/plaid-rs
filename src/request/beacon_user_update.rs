use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::BeaconUserUpdateRequestData;
/**You should use this struct via [`PlaidClient::beacon_user_update`].

On request success, this will return a [`BeaconUserUpdateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconUserUpdateRequest {
    pub access_tokens: Option<Vec<String>>,
    pub beacon_user_id: String,
    pub user: Option<BeaconUserUpdateRequestData>,
}
impl FluentRequest<'_, BeaconUserUpdateRequest> {
    ///Set the value of the access_tokens field.
    pub fn access_tokens(
        mut self,
        access_tokens: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .access_tokens = Some(
            access_tokens.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    ///Set the value of the user field.
    pub fn user(mut self, user: BeaconUserUpdateRequestData) -> Self {
        self.params.user = Some(user);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BeaconUserUpdateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::BeaconUserUpdateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beacon/user/update";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.access_tokens {
                r = r.json(serde_json::json!({ "access_tokens" : unwrapped }));
            }
            r = r
                .json(
                    serde_json::json!({ "beacon_user_id" : self.params.beacon_user_id }),
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
    /**Update the identity data of a Beacon User

Update the identity data for a Beacon User in your Beacon Program or add new accounts to the Beacon User.

Similar to `/beacon/user/create`, several checks are performed immediately when you submit an identity data change to `/beacon/user/update`:

  - The user's updated PII is searched against all other users within the Beacon Program you specified. If a match is found that violates your program's "Duplicate Information Filtering" settings, the user will be returned with a status of `pending_review`.

  - The user's updated PII is also searched against all fraud reports created by your organization across all of your Beacon Programs. If the user's data matches a fraud report that your team created, the user will be returned with a status of `rejected`.

  - Finally, the user's PII is searched against all fraud report shared with the Beacon Network by other companies. If a matching fraud report is found, the user will be returned with a `pending_review` status if your program has enabled automatic flagging based on network fraud.

Plaid maintains a version history for each Beacon User, so the Beacon User's identity data before and after the update is retained as separate versions.

See endpoint docs at <https://plaid.com/docs/api/products/beacon/#beaconuserupdate>.*/
    pub fn beacon_user_update(
        &self,
        beacon_user_id: &str,
    ) -> FluentRequest<'_, BeaconUserUpdateRequest> {
        FluentRequest {
            client: self,
            params: BeaconUserUpdateRequest {
                access_tokens: None,
                beacon_user_id: beacon_user_id.to_owned(),
                user: None,
            },
        }
    }
}
