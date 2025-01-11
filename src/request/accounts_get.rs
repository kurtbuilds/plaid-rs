use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::AccountsGetRequestOptions;
/**You should use this struct via [`PlaidClient::accounts_get`].

On request success, this will return a [`AccountsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountsGetRequest {
    pub access_token: String,
    pub options: Option<AccountsGetRequestOptions>,
}
impl FluentRequest<'_, AccountsGetRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: AccountsGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AccountsGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::AccountsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/accounts/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
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
    /**Retrieve accounts

The `/accounts/get` endpoint can be used to retrieve a list of accounts associated with any linked Item. Plaid will only return active bank accounts — that is, accounts that are not closed and are capable of carrying a balance.
To return new accounts that were created after the user linked their Item, you can listen for the [`NEW_ACCOUNTS_AVAILABLE`](https://plaid.com/docs/api/items/#new_accounts_available) webhook and then use Link's [update mode](https://plaid.com/docs/link/update-mode/) to request that the user share this new account with you.

`/accounts/get` is free to use and retrieves cached information, rather than extracting fresh information from the institution. The balance returned will reflect the balance at the time of the last successful Item update. If the Item is enabled for a regularly updating product, such as Transactions, Investments, or Liabilities, the balance will typically update about once a day, as long as the Item is healthy. If the Item is enabled only for products that do not frequently update, such as Auth or Identity, balance data may be much older.

For realtime balance information, use the paid endpoint `/accounts/balance/get` instead.

See endpoint docs at <https://plaid.com/docs/api/accounts/#accountsget>.*/
    pub fn accounts_get(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, AccountsGetRequest> {
        FluentRequest {
            client: self,
            params: AccountsGetRequest {
                access_token: access_token.to_owned(),
                options: None,
            },
        }
    }
}
