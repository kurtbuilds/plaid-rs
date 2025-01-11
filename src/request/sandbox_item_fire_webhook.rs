use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::WebhookType;
/**You should use this struct via [`PlaidClient::sandbox_item_fire_webhook`].

On request success, this will return a [`SandboxItemFireWebhookResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxItemFireWebhookRequest {
    pub access_token: String,
    pub webhook_code: String,
    pub webhook_type: Option<WebhookType>,
}
impl FluentRequest<'_, SandboxItemFireWebhookRequest> {
    ///Set the value of the webhook_type field.
    pub fn webhook_type(mut self, webhook_type: WebhookType) -> Self {
        self.params.webhook_type = Some(webhook_type);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, SandboxItemFireWebhookRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::SandboxItemFireWebhookResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/item/fire_webhook";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = r.json(serde_json::json!({ "webhook_code" : self.params.webhook_code }));
            if let Some(ref unwrapped) = self.params.webhook_type {
                r = r.json(serde_json::json!({ "webhook_type" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Fire a test webhook

The `/sandbox/item/fire_webhook` endpoint is used to test that code correctly handles webhooks. This endpoint can trigger the following webhooks:

`DEFAULT_UPDATE`: Webhook to be fired for a given Sandbox Item simulating a default update event for the respective product as specified with the `webhook_type` in the request body. Valid Sandbox `DEFAULT_UPDATE` webhook types include: `AUTH`, `IDENTITY`, `TRANSACTIONS`, `INVESTMENTS_TRANSACTIONS`, `LIABILITIES`, `HOLDINGS`. If the Item does not support the product, a `SANDBOX_PRODUCT_NOT_ENABLED` error will result.

`NEW_ACCOUNTS_AVAILABLE`: Fired to indicate that a new account is available on the Item and you can launch update mode to request access to it.

`SMS_MICRODEPOSITS_VERIFICATION`: Fired when a given same day micro-deposit item is verified via SMS verification.

`LOGIN_REPAIRED`: Fired when an Item recovers from the `ITEM_LOGIN_REQUIRED` without the user going through update mode in your app.

`PENDING_DISCONNECT`: Fired when an Item will stop working in the near future (e.g. due to a planned bank migration) and must be sent through update mode to continue working.

`RECURRING_TRANSACTIONS_UPDATE`: Recurring Transactions webhook to be fired for a given Sandbox Item. If the Item does not support Recurring Transactions, a `SANDBOX_PRODUCT_NOT_ENABLED` error will result.

`SYNC_UPDATES_AVAILABLE`: Transactions webhook to be fired for a given Sandbox Item.  If the Item does not support Transactions, a `SANDBOX_PRODUCT_NOT_ENABLED` error will result.

`PRODUCT_READY`: Assets webhook to be fired when a given asset report has been successfully generated. If the Item does not support Assets, a `SANDBOX_PRODUCT_NOT_ENABLED` error will result.

`ERROR`: Assets webhook to be fired when asset report generation has failed. If the Item does not support Assets, a `SANDBOX_PRODUCT_NOT_ENABLED` error will result.

Note that this endpoint is provided for developer ease-of-use and is not required for testing webhooks; webhooks will also fire in Sandbox under the same conditions that they would in Production (except for webhooks of type `TRANSFER`).

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxitemfire_webhook>.*/
    pub fn sandbox_item_fire_webhook(
        &self,
        access_token: &str,
        webhook_code: &str,
    ) -> FluentRequest<'_, SandboxItemFireWebhookRequest> {
        FluentRequest {
            client: self,
            params: SandboxItemFireWebhookRequest {
                access_token: access_token.to_owned(),
                webhook_code: webhook_code.to_owned(),
                webhook_type: None,
            },
        }
    }
}
