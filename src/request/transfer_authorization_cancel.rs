use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transfer_authorization_cancel`].

On request success, this will return a [`TransferAuthorizationCancelResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferAuthorizationCancelRequest {
    pub authorization_id: String,
}
impl FluentRequest<'_, TransferAuthorizationCancelRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferAuthorizationCancelRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransferAuthorizationCancelResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/authorization/cancel";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "authorization_id" : self.params.authorization_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Cancel a transfer authorization

Use the `/transfer/authorization/cancel` endpoint to cancel a transfer authorization. A transfer authorization is eligible for cancellation if it has not yet been used to create a transfer.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/initiating-transfers/#transferauthorizationcancel>.*/
    pub fn transfer_authorization_cancel(
        &self,
        authorization_id: &str,
    ) -> FluentRequest<'_, TransferAuthorizationCancelRequest> {
        FluentRequest {
            client: self,
            params: TransferAuthorizationCancelRequest {
                authorization_id: authorization_id.to_owned(),
            },
        }
    }
}
