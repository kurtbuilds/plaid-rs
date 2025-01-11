use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::BeaconUserRequestData;
/**You should use this struct via [`PlaidClient::beacon_user_create`].

On request success, this will return a [`BeaconUserCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconUserCreateRequest {
    pub access_tokens: Option<Vec<String>>,
    pub client_user_id: String,
    pub program_id: String,
    pub user: BeaconUserRequestData,
}
impl FluentRequest<'_, BeaconUserCreateRequest> {
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
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BeaconUserCreateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::BeaconUserCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beacon/user/create";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.access_tokens {
                r = r.json(serde_json::json!({ "access_tokens" : unwrapped }));
            }
            r = r
                .json(
                    serde_json::json!({ "client_user_id" : self.params.client_user_id }),
                );
            r = r.json(serde_json::json!({ "program_id" : self.params.program_id }));
            r = r.json(serde_json::json!({ "user" : self.params.user }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create a Beacon User

Create and scan a Beacon User against your Beacon Program, according to your program's settings.

When you submit a new user to `/beacon/user/create`, several checks are performed immediately:

  - The user's PII (provided within the `user` object) is searched against all other users within the Beacon Program you specified. If a match is found that violates your program's "Duplicate Information Filtering" settings, the user will be returned with a status of `pending_review`.

  - The user's PII is also searched against all fraud reports created by your organization across all of your Beacon Programs. If the user's data matches a fraud report that your team created, the user will be returned with a status of `rejected`.

  - Finally, the user's PII is searched against all fraud report shared with the Beacon Network by other companies. If a matching fraud report is found, the user will be returned with a `pending_review` status if your program has enabled automatic flagging based on network fraud.

See endpoint docs at <https://plaid.com/docs/api/products/beacon/#beaconusercreate>.*/
    pub fn beacon_user_create(
        &self,
        client_user_id: &str,
        program_id: &str,
        user: BeaconUserRequestData,
    ) -> FluentRequest<'_, BeaconUserCreateRequest> {
        FluentRequest {
            client: self,
            params: BeaconUserCreateRequest {
                access_tokens: None,
                client_user_id: client_user_id.to_owned(),
                program_id: program_id.to_owned(),
                user,
            },
        }
    }
}
