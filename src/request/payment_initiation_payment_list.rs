use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::payment_initiation_payment_list`].

On request success, this will return a [`PaymentInitiationPaymentListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentListRequest {
    pub consent_id: Option<String>,
    pub count: Option<i64>,
    pub cursor: Option<chrono::DateTime<chrono::Utc>>,
}
impl FluentRequest<'_, PaymentInitiationPaymentListRequest> {
    ///Set the value of the consent_id field.
    pub fn consent_id(mut self, consent_id: &str) -> Self {
        self.params.consent_id = Some(consent_id.to_owned());
        self
    }
    ///Set the value of the count field.
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
    ///Set the value of the cursor field.
    pub fn cursor(mut self, cursor: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.cursor = Some(cursor);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PaymentInitiationPaymentListRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::PaymentInitiationPaymentListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/payment_initiation/payment/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.consent_id {
                r = r.json(serde_json::json!({ "consent_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.count {
                r = r.json(serde_json::json!({ "count" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.cursor {
                r = r.json(serde_json::json!({ "cursor" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**List payments

The `/payment_initiation/payment/list` endpoint can be used to retrieve all created payments. By default, the 10 most recent payments are returned. You can request more payments and paginate through the results using the optional `count` and `cursor` parameters.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationpaymentlist>.*/
    pub fn payment_initiation_payment_list(
        &self,
    ) -> FluentRequest<'_, PaymentInitiationPaymentListRequest> {
        FluentRequest {
            client: self,
            params: PaymentInitiationPaymentListRequest {
                consent_id: None,
                count: None,
                cursor: None,
            },
        }
    }
}
