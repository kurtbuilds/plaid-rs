use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::TransferFundingAccount;
/**You should use this struct via [`PlaidClient::transfer_originator_funding_account_update`].

On request success, this will return a [`TransferOriginatorFundingAccountUpdateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferOriginatorFundingAccountUpdateRequest {
    pub funding_account: TransferFundingAccount,
    pub originator_client_id: String,
}
impl FluentRequest<'_, TransferOriginatorFundingAccountUpdateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferOriginatorFundingAccountUpdateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransferOriginatorFundingAccountUpdateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/originator/funding_account/update";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "funding_account" : self.params.funding_account }
                    ),
                );
            r = r
                .json(
                    serde_json::json!(
                        { "originator_client_id" : self.params.originator_client_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Update the funding account associated with the originator

Use the `/transfer/originator/funding_account/update` endpoint to update the funding account associated with the originator.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/platform-payments/#transferoriginatorfunding_accountupdate>.*/
    pub fn transfer_originator_funding_account_update(
        &self,
        funding_account: TransferFundingAccount,
        originator_client_id: &str,
    ) -> FluentRequest<'_, TransferOriginatorFundingAccountUpdateRequest> {
        FluentRequest {
            client: self,
            params: TransferOriginatorFundingAccountUpdateRequest {
                funding_account,
                originator_client_id: originator_client_id.to_owned(),
            },
        }
    }
}
