use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::bank_transfer_cancel`].

On request success, this will return a [`BankTransferCancelResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferCancelRequest {
    pub bank_transfer_id: String,
}
impl FluentRequest<'_, BankTransferCancelRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BankTransferCancelRequest> {
    type Output = httpclient::InMemoryResult<crate::model::BankTransferCancelResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/bank_transfer/cancel";
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
    /**Cancel a bank transfer

Use the `/bank_transfer/cancel` endpoint to cancel a bank transfer.  A transfer is eligible for cancelation if the `cancellable` property returned by `/bank_transfer/get` is `true`.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transfercancel>.*/
    pub fn bank_transfer_cancel(
        &self,
        bank_transfer_id: &str,
    ) -> FluentRequest<'_, BankTransferCancelRequest> {
        FluentRequest {
            client: self,
            params: BankTransferCancelRequest {
                bank_transfer_id: bank_transfer_id.to_owned(),
            },
        }
    }
}
