use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::wallet_get`].

On request success, this will return a [`WalletGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletGetRequest {
    pub wallet_id: String,
}
impl FluentRequest<'_, WalletGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, WalletGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::WalletGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/wallet/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "wallet_id" : self.params.wallet_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Fetch an e-wallet

Fetch an e-wallet. The response includes the current balance.

See endpoint docs at <https://plaid.com/docs/api/products/virtual-accounts/#walletget>.*/
    pub fn wallet_get(&self, wallet_id: &str) -> FluentRequest<'_, WalletGetRequest> {
        FluentRequest {
            client: self,
            params: WalletGetRequest {
                wallet_id: wallet_id.to_owned(),
            },
        }
    }
}
