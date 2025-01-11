use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::DepositSwitchCreateRequestOptions;
/**You should use this struct via [`PlaidClient::deposit_switch_create`].

On request success, this will return a [`DepositSwitchCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositSwitchCreateRequest {
    pub country_code: Option<String>,
    pub options: Option<DepositSwitchCreateRequestOptions>,
    pub target_access_token: String,
    pub target_account_id: String,
}
impl FluentRequest<'_, DepositSwitchCreateRequest> {
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
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, DepositSwitchCreateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::DepositSwitchCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/deposit_switch/create";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.country_code {
                r = r.json(serde_json::json!({ "country_code" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
            r = r
                .json(
                    serde_json::json!(
                        { "target_access_token" : self.params.target_access_token }
                    ),
                );
            r = r
                .json(
                    serde_json::json!(
                        { "target_account_id" : self.params.target_account_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**(Deprecated) Create a deposit switch

This endpoint creates a deposit switch entity that will be persisted throughout the lifecycle of the switch.

See endpoint docs at <https://plaid.com/docs/deposit-switch/reference#deposit_switchcreate>.*/
    pub fn deposit_switch_create(
        &self,
        target_access_token: &str,
        target_account_id: &str,
    ) -> FluentRequest<'_, DepositSwitchCreateRequest> {
        FluentRequest {
            client: self,
            params: DepositSwitchCreateRequest {
                country_code: None,
                options: None,
                target_access_token: target_access_token.to_owned(),
                target_account_id: target_account_id.to_owned(),
            },
        }
    }
}
