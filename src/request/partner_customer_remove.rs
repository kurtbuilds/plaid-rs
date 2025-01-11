use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::partner_customer_remove`].

On request success, this will return a [`PartnerCustomerRemoveResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerCustomerRemoveRequest {
    pub client_id: Option<String>,
    pub end_customer_client_id: String,
    pub secret: Option<String>,
}
impl FluentRequest<'_, PartnerCustomerRemoveRequest> {
    ///Set the value of the client_id field.
    pub fn client_id(mut self, client_id: &str) -> Self {
        self.params.client_id = Some(client_id.to_owned());
        self
    }
    ///Set the value of the secret field.
    pub fn secret(mut self, secret: &str) -> Self {
        self.params.secret = Some(secret.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PartnerCustomerRemoveRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::PartnerCustomerRemoveResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/partner/customer/remove";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.client_id {
                r = r.json(serde_json::json!({ "client_id" : unwrapped }));
            }
            r = r
                .json(
                    serde_json::json!(
                        { "end_customer_client_id" : self.params.end_customer_client_id }
                    ),
                );
            if let Some(ref unwrapped) = self.params.secret {
                r = r.json(serde_json::json!({ "secret" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Removes a Plaid reseller's end customer.

The `/partner/customer/remove` endpoint is used by reseller partners to remove an end customer. Removing an end customer will remove it from view in the Plaid Dashboard and deactivate its API keys. This endpoint can only be used to remove an end customer that has not yet been enabled in full Production.

See endpoint docs at <https://plaid.com/docs/api/partner/#partnercustomerremove>.*/
    pub fn partner_customer_remove(
        &self,
        end_customer_client_id: &str,
    ) -> FluentRequest<'_, PartnerCustomerRemoveRequest> {
        FluentRequest {
            client: self,
            params: PartnerCustomerRemoveRequest {
                client_id: None,
                end_customer_client_id: end_customer_client_id.to_owned(),
                secret: None,
            },
        }
    }
}
