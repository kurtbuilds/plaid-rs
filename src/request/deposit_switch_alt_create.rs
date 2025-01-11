use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{
    DepositSwitchCreateRequestOptions, DepositSwitchTargetAccount,
    DepositSwitchTargetUser,
};
/**You should use this struct via [`PlaidClient::deposit_switch_alt_create`].

On request success, this will return a [`DepositSwitchAltCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositSwitchAltCreateRequest {
    pub country_code: Option<String>,
    pub options: Option<DepositSwitchCreateRequestOptions>,
    pub target_account: DepositSwitchTargetAccount,
    pub target_user: DepositSwitchTargetUser,
}
impl FluentRequest<'_, DepositSwitchAltCreateRequest> {
    ///Set the value of the country_code field.
    pub fn country_code(mut self, country_code: &str) -> Self {
        self.params.country_code = Some(country_code.to_owned());
        self
    }
    ///Set the value of the options field.
    pub fn options(mut self, options: DepositSwitchCreateRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, DepositSwitchAltCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::DepositSwitchAltCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/deposit_switch/alt/create";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.country_code {
                r = r.json(serde_json::json!({ "country_code" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
            r = r
                .json(
                    serde_json::json!({ "target_account" : self.params.target_account }),
                );
            r = r.json(serde_json::json!({ "target_user" : self.params.target_user }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**(Deprecated) Create a deposit switch without using Plaid Exchange

This endpoint provides an alternative to `/deposit_switch/create` for customers who have not yet fully integrated with Plaid Exchange. Like `/deposit_switch/create`, it creates a deposit switch entity that will be persisted throughout the lifecycle of the switch.

See endpoint docs at <https://plaid.com/docs/deposit-switch/reference#deposit_switchaltcreate>.*/
    pub fn deposit_switch_alt_create(
        &self,
        target_account: DepositSwitchTargetAccount,
        target_user: DepositSwitchTargetUser,
    ) -> FluentRequest<'_, DepositSwitchAltCreateRequest> {
        FluentRequest {
            client: self,
            params: DepositSwitchAltCreateRequest {
                country_code: None,
                options: None,
                target_account,
                target_user,
            },
        }
    }
}
