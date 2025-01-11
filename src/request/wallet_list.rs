use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::WalletIsoCurrencyCode;
/**You should use this struct via [`PlaidClient::wallet_list`].

On request success, this will return a [`WalletListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletListRequest {
    pub count: Option<i64>,
    pub cursor: Option<String>,
    pub iso_currency_code: Option<WalletIsoCurrencyCode>,
}
impl FluentRequest<'_, WalletListRequest> {
    ///Set the value of the count field.
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
    ///Set the value of the cursor field.
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
    ///Set the value of the iso_currency_code field.
    pub fn iso_currency_code(
        mut self,
        iso_currency_code: WalletIsoCurrencyCode,
    ) -> Self {
        self.params.iso_currency_code = Some(iso_currency_code);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, WalletListRequest> {
    type Output = httpclient::InMemoryResult<crate::model::WalletListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/wallet/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.count {
                r = r.json(serde_json::json!({ "count" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.cursor {
                r = r.json(serde_json::json!({ "cursor" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.iso_currency_code {
                r = r.json(serde_json::json!({ "iso_currency_code" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Fetch a list of e-wallets

This endpoint lists all e-wallets in descending order of creation.

See endpoint docs at <https://plaid.com/docs/api/products/virtual-accounts/#walletlist>.*/
    pub fn wallet_list(&self) -> FluentRequest<'_, WalletListRequest> {
        FluentRequest {
            client: self,
            params: WalletListRequest {
                count: None,
                cursor: None,
                iso_currency_code: None,
            },
        }
    }
}
