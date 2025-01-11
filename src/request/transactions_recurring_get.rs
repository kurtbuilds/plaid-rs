use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::TransactionsRecurringGetRequestOptions;
/**You should use this struct via [`PlaidClient::transactions_recurring_get`].

On request success, this will return a [`TransactionsRecurringGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsRecurringGetRequest {
    pub access_token: String,
    pub account_ids: Option<Vec<String>>,
    pub options: Option<TransactionsRecurringGetRequestOptions>,
}
impl FluentRequest<'_, TransactionsRecurringGetRequest> {
    ///Set the value of the account_ids field.
    pub fn account_ids(
        mut self,
        account_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .account_ids = Some(
            account_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    ///Set the value of the options field.
    pub fn options(mut self, options: TransactionsRecurringGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransactionsRecurringGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransactionsRecurringGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transactions/recurring/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            if let Some(ref unwrapped) = self.params.account_ids {
                r = r.json(serde_json::json!({ "account_ids" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Fetch recurring transaction streams

The `/transactions/recurring/get` endpoint allows developers to receive a summary of the recurring outflow and inflow streams (expenses and deposits) from a user’s checking, savings or credit card accounts. Additionally, Plaid provides key insights about each recurring stream including the category, merchant, last amount, and more. Developers can use these insights to build tools and experiences that help their users better manage cash flow, monitor subscriptions, reduce spend, and stay on track with bill payments.

This endpoint is offered as an add-on to Transactions. To request access to this endpoint, submit a [product access request](https://dashboard.plaid.com/team/products) or contact your Plaid account manager.

This endpoint can only be called on an Item that has already been initialized with Transactions (either during Link, by specifying it in `/link/token/create`; or after Link, by calling `/transactions/get` or `/transactions/sync`).

When using Recurring Transactions, for best results, make sure to use the [`days_requested`](https://plaid.com/docs/api/link/#link-token-create-request-transactions-days-requested) parameter to request at least 180 days of history when initializing Items with Transactions. Once all historical transactions have been fetched, call `/transactions/recurring/get` to receive the Recurring Transactions streams and subscribe to the [`RECURRING_TRANSACTIONS_UPDATE`](https://plaid.com/docs/api/products/transactions/#recurring_transactions_update) webhook. To know when historical transactions have been fetched, if you are using `/transactions/sync` listen for the [`SYNC_UPDATES_AVAILABLE`](https://plaid.com/docs/api/products/transactions/#SyncUpdatesAvailableWebhook-historical-update-complete) webhook and check that the `historical_update_complete` field in the payload is `true`. If using `/transactions/get`, listen for the [`HISTORICAL_UPDATE`](https://plaid.com/docs/api/products/transactions/#historical_update) webhook.

After the initial call, you can call `/transactions/recurring/get` endpoint at any point in the future to retrieve the latest summary of recurring streams. Listen to the [`RECURRING_TRANSACTIONS_UPDATE`](https://plaid.com/docs/api/products/transactions/#recurring_transactions_update) webhook to be notified when new updates are available.

See endpoint docs at <https://plaid.com/docs/api/products/transactions/#transactionsrecurringget>.*/
    pub fn transactions_recurring_get(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, TransactionsRecurringGetRequest> {
        FluentRequest {
            client: self,
            params: TransactionsRecurringGetRequest {
                access_token: access_token.to_owned(),
                account_ids: None,
                options: None,
            },
        }
    }
}
