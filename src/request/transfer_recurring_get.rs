use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transfer_recurring_get`].

On request success, this will return a [`TransferRecurringGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRecurringGetRequest {
    pub recurring_transfer_id: String,
}
impl FluentRequest<'_, TransferRecurringGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferRecurringGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::TransferRecurringGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/recurring/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "recurring_transfer_id" : self.params.recurring_transfer_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve a recurring transfer

The `/transfer/recurring/get` fetches information about the recurring transfer corresponding to the given `recurring_transfer_id`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/recurring-transfers/#transferrecurringget>.*/
    pub fn transfer_recurring_get(
        &self,
        recurring_transfer_id: &str,
    ) -> FluentRequest<'_, TransferRecurringGetRequest> {
        FluentRequest {
            client: self,
            params: TransferRecurringGetRequest {
                recurring_transfer_id: recurring_transfer_id.to_owned(),
            },
        }
    }
}
