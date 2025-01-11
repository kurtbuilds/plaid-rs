use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::deposit_switch_get`].

On request success, this will return a [`DepositSwitchGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositSwitchGetRequest {
    pub deposit_switch_id: String,
}
impl FluentRequest<'_, DepositSwitchGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, DepositSwitchGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::DepositSwitchGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/deposit_switch/get";
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
    /**(Deprecated) Retrieve a deposit switch

This endpoint returns information related to how the user has configured their payroll allocation and the state of the switch. You can use this information to build logic related to the user's direct deposit allocation preferences.

See endpoint docs at <https://plaid.com/docs/deposit-switch/reference#deposit_switchget>.*/
    pub fn deposit_switch_get(
        &self,
        deposit_switch_id: &str,
    ) -> FluentRequest<'_, DepositSwitchGetRequest> {
        FluentRequest {
            client: self,
            params: DepositSwitchGetRequest {
                deposit_switch_id: deposit_switch_id.to_owned(),
            },
        }
    }
}
