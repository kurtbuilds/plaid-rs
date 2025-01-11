use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::WalletIsoCurrencyCode;
/**You should use this struct via [`PlaidClient::wallet_create`].

On request success, this will return a [`WalletCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletCreateRequest {
    pub iso_currency_code: WalletIsoCurrencyCode,
}
impl FluentRequest<'_, WalletCreateRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, WalletCreateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::WalletCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/wallet/create";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "iso_currency_code" : self.params.iso_currency_code }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create an e-wallet

Create an e-wallet. The response is the newly created e-wallet object.

See endpoint docs at <https://plaid.com/docs/api/products/virtual-accounts/#walletcreate>.*/
    pub fn wallet_create(
        &self,
        iso_currency_code: WalletIsoCurrencyCode,
    ) -> FluentRequest<'_, WalletCreateRequest> {
        FluentRequest {
            client: self,
            params: WalletCreateRequest {
                iso_currency_code,
            },
        }
    }
}
