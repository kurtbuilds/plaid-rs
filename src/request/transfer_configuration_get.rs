use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transfer_configuration_get`].

On request success, this will return a [`TransferConfigurationGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferConfigurationGetRequest {
    pub originator_client_id: Option<String>,
}
impl FluentRequest<'_, TransferConfigurationGetRequest> {
    ///Set the value of the originator_client_id field.
    pub fn originator_client_id(mut self, originator_client_id: &str) -> Self {
        self.params.originator_client_id = Some(originator_client_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferConfigurationGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransferConfigurationGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/configuration/get";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.originator_client_id {
                r = r.json(serde_json::json!({ "originator_client_id" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get transfer product configuration

Use the `/transfer/configuration/get` endpoint to view your transfer product configurations.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/metrics/#transferconfigurationget>.*/
    pub fn transfer_configuration_get(
        &self,
    ) -> FluentRequest<'_, TransferConfigurationGetRequest> {
        FluentRequest {
            client: self,
            params: TransferConfigurationGetRequest {
                originator_client_id: None,
            },
        }
    }
}
