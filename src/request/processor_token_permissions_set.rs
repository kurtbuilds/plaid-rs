use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::Products;
/**You should use this struct via [`PlaidClient::processor_token_permissions_set`].

On request success, this will return a [`ProcessorTokenPermissionsSetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorTokenPermissionsSetRequest {
    pub processor_token: String,
    pub products: Vec<Products>,
}
impl FluentRequest<'_, ProcessorTokenPermissionsSetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProcessorTokenPermissionsSetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::ProcessorTokenPermissionsSetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/token/permissions/set";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "processor_token" : self.params.processor_token }
                    ),
                );
            r = r.json(serde_json::json!({ "products" : self.params.products }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Control a processor's access to products

Used to control a processor's access to products on the given processor token. By default, a processor will have access to all available products on the corresponding item. To restrict access to a particular set of products, call this endpoint with the desired products. To restore access to all available products, call this endpoint with an empty list. This endpoint can be called multiple times as your needs and your processor's needs change.

See endpoint docs at <https://plaid.com/docs/api/processors/#processortokenpermissionsset>.*/
    pub fn processor_token_permissions_set(
        &self,
        processor_token: &str,
        products: Vec<Products>,
    ) -> FluentRequest<'_, ProcessorTokenPermissionsSetRequest> {
        FluentRequest {
            client: self,
            params: ProcessorTokenPermissionsSetRequest {
                processor_token: processor_token.to_owned(),
                products,
            },
        }
    }
}
