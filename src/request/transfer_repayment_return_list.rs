use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transfer_repayment_return_list`].

On request success, this will return a [`TransferRepaymentReturnListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRepaymentReturnListRequest {
    pub count: Option<i64>,
    pub offset: Option<i64>,
    pub repayment_id: String,
}
impl FluentRequest<'_, TransferRepaymentReturnListRequest> {
    ///Set the value of the count field.
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
    ///Set the value of the offset field.
    pub fn offset(mut self, offset: i64) -> Self {
        self.params.offset = Some(offset);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferRepaymentReturnListRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransferRepaymentReturnListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/repayment/return/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.count {
                r = r.json(serde_json::json!({ "count" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.offset {
                r = r.json(serde_json::json!({ "offset" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "repayment_id" : self.params.repayment_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**List the returns included in a repayment

The `/transfer/repayment/return/list` endpoint retrieves the set of returns that were batched together into the specified repayment. The sum of amounts of returns retrieved by this request equals the amount of the repayment.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferrepaymentreturnlist>.*/
    pub fn transfer_repayment_return_list(
        &self,
        repayment_id: &str,
    ) -> FluentRequest<'_, TransferRepaymentReturnListRequest> {
        FluentRequest {
            client: self,
            params: TransferRepaymentReturnListRequest {
                count: None,
                offset: None,
                repayment_id: repayment_id.to_owned(),
            },
        }
    }
}
