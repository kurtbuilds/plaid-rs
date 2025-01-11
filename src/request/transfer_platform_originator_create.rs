use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::TransferPlatformTosAcceptanceMetadata;
/**You should use this struct via [`PlaidClient::transfer_platform_originator_create`].

On request success, this will return a [`TransferPlatformOriginatorCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferPlatformOriginatorCreateRequest {
    pub originator_client_id: String,
    pub originator_reviewed_at: chrono::DateTime<chrono::Utc>,
    pub tos_acceptance_metadata: TransferPlatformTosAcceptanceMetadata,
}
impl FluentRequest<'_, TransferPlatformOriginatorCreateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferPlatformOriginatorCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransferPlatformOriginatorCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/platform/originator/create";
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
                        { "originator_reviewed_at" : self.params.originator_reviewed_at }
                    ),
                );
            r = r
                .json(
                    serde_json::json!(
                        { "tos_acceptance_metadata" : self.params.tos_acceptance_metadata
                        }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create an originator for scaled platform customers

The `/transfer/platform/originator/create` endpoint allows gathering information about the originator specific to the Scaled Platform Transfer offering, including the originator's agreement to legal terms required before accepting any further information related to the originator.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/platform/originator/#transferplatformoriginatorcreate>.*/
    pub fn transfer_platform_originator_create(
        &self,
        originator_client_id: &str,
        originator_reviewed_at: chrono::DateTime<chrono::Utc>,
        tos_acceptance_metadata: TransferPlatformTosAcceptanceMetadata,
    ) -> FluentRequest<'_, TransferPlatformOriginatorCreateRequest> {
        FluentRequest {
            client: self,
            params: TransferPlatformOriginatorCreateRequest {
                originator_client_id: originator_client_id.to_owned(),
                originator_reviewed_at,
                tos_acceptance_metadata,
            },
        }
    }
}
