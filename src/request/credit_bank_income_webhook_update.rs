use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::credit_bank_income_webhook_update`].

On request success, this will return a [`CreditBankIncomeWebhookUpdateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditBankIncomeWebhookUpdateRequest {
    pub enable_webhooks: bool,
    pub user_token: String,
}
impl FluentRequest<'_, CreditBankIncomeWebhookUpdateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreditBankIncomeWebhookUpdateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::CreditBankIncomeWebhookUpdateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/bank_income/webhook/update";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "enable_webhooks" : self.params.enable_webhooks }
                    ),
                );
            r = r.json(serde_json::json!({ "user_token" : self.params.user_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Subscribe and unsubscribe to proactive notifications for a user's income profile

`/credit/bank_income/webhook/update` allows you to subscribe or unsubscribe a user for income webhook notifications. By default, all users start out unsubscribed.

If a user is subscribed, on significant changes to the user's income profile, you will receive a `BANK_INCOME_REFRESH_UPDATE` webhook, prompting you to refresh bank income data for the user.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditbank_incomewebhookupdate>.*/
    pub fn credit_bank_income_webhook_update(
        &self,
        enable_webhooks: bool,
        user_token: &str,
    ) -> FluentRequest<'_, CreditBankIncomeWebhookUpdateRequest> {
        FluentRequest {
            client: self,
            params: CreditBankIncomeWebhookUpdateRequest {
                enable_webhooks,
                user_token: user_token.to_owned(),
            },
        }
    }
}
