use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::TransferOriginatorDiligence;
/**You should use this struct via [`PlaidClient::transfer_diligence_submit`].

On request success, this will return a [`TransferDiligenceSubmitResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferDiligenceSubmitRequest {
    pub originator_client_id: String,
    pub originator_diligence: TransferOriginatorDiligence,
}
impl FluentRequest<'_, TransferDiligenceSubmitRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferDiligenceSubmitRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransferDiligenceSubmitResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/diligence/submit";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "originator_client_id" : self.params.originator_client_id }
                    ),
                );
            r = r
                .json(
                    serde_json::json!(
                        { "originator_diligence" : self.params.originator_diligence }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Submit transfer diligence on behalf of the originator

Use the `/transfer/diligence/submit` endpoint to submit transfer diligence on behalf of the originator (i.e., the end customer).

See endpoint docs at <https://plaid.com/docs/api/products/transfer/platform-payments/#transferdiligencesubmit>.*/
    pub fn transfer_diligence_submit(
        &self,
        originator_client_id: &str,
        originator_diligence: TransferOriginatorDiligence,
    ) -> FluentRequest<'_, TransferDiligenceSubmitRequest> {
        FluentRequest {
            client: self,
            params: TransferDiligenceSubmitRequest {
                originator_client_id: originator_client_id.to_owned(),
                originator_diligence,
            },
        }
    }
}
