use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::bank_transfer_get`].

On request success, this will return a [`BankTransferGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferGetRequest {
    pub bank_transfer_id: String,
}
impl FluentRequest<'_, BankTransferGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BankTransferGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::BankTransferGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/bank_transfer/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "bank_transfer_id" : self.params.bank_transfer_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve a bank transfer

The `/bank_transfer/get` fetches information about the bank transfer corresponding to the given `bank_transfer_id`.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transferget>.*/
    pub fn bank_transfer_get(
        &self,
        bank_transfer_id: &str,
    ) -> FluentRequest<'_, BankTransferGetRequest> {
        FluentRequest {
            client: self,
            params: BankTransferGetRequest {
                bank_transfer_id: bank_transfer_id.to_owned(),
            },
        }
    }
}
