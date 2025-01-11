use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::partner_customer_get`].

On request success, this will return a [`PartnerCustomerGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerCustomerGetRequest {
    pub client_id: Option<String>,
    pub end_customer_client_id: String,
    pub secret: Option<String>,
}
impl FluentRequest<'_, PartnerCustomerGetRequest> {
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
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PartnerCustomerGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::PartnerCustomerGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/partner/customer/get";
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
    /**Returns a Plaid reseller's end customer.

The `/partner/customer/get` endpoint is used by reseller partners to retrieve data about a single end customer.

See endpoint docs at <https://plaid.com/docs/api/partner/#partnercustomerget>.*/
    pub fn partner_customer_get(
        &self,
        end_customer_client_id: &str,
    ) -> FluentRequest<'_, PartnerCustomerGetRequest> {
        FluentRequest {
            client: self,
            params: PartnerCustomerGetRequest {
                client_id: None,
                end_customer_client_id: end_customer_client_id.to_owned(),
                secret: None,
            },
        }
    }
}
