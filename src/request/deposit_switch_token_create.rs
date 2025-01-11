use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::deposit_switch_token_create`].

On request success, this will return a [`DepositSwitchTokenCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositSwitchTokenCreateRequest {
    pub deposit_switch_id: String,
}
impl FluentRequest<'_, DepositSwitchTokenCreateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, DepositSwitchTokenCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::DepositSwitchTokenCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/deposit_switch/token/create";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "deposit_switch_id" : self.params.deposit_switch_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**(Deprecated) Create a deposit switch token

In order for the end user to take action, you will need to create a public token representing the deposit switch. This token is used to initialize Link. It can be used one time and expires after 30 minutes.


See endpoint docs at <https://plaid.com/docs/deposit-switch/reference#deposit_switchtokencreate>.*/
    pub fn deposit_switch_token_create(
        &self,
        deposit_switch_id: &str,
    ) -> FluentRequest<'_, DepositSwitchTokenCreateRequest> {
        FluentRequest {
            client: self,
            params: DepositSwitchTokenCreateRequest {
                deposit_switch_id: deposit_switch_id.to_owned(),
            },
        }
    }
}
