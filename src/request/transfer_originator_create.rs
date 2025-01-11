use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transfer_originator_create`].

On request success, this will return a [`TransferOriginatorCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferOriginatorCreateRequest {
    pub company_name: String,
}
impl FluentRequest<'_, TransferOriginatorCreateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferOriginatorCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransferOriginatorCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/originator/create";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "company_name" : self.params.company_name }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create a new originator

Use the `/transfer/originator/create` endpoint to create a new originator and return an `originator_client_id`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/platform-payments/#transferoriginatorcreate>.*/
    pub fn transfer_originator_create(
        &self,
        company_name: &str,
    ) -> FluentRequest<'_, TransferOriginatorCreateRequest> {
        FluentRequest {
            client: self,
            params: TransferOriginatorCreateRequest {
                company_name: company_name.to_owned(),
            },
        }
    }
}
