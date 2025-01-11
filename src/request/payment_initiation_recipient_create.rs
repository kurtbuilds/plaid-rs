use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{PaymentInitiationAddress, RecipientBACSNullable};
/**You should use this struct via [`PlaidClient::payment_initiation_recipient_create`].

On request success, this will return a [`PaymentInitiationRecipientCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientCreateRequest {
    pub address: Option<PaymentInitiationAddress>,
    pub bacs: Option<RecipientBacsNullable>,
    pub iban: Option<String>,
    pub name: String,
}
impl FluentRequest<'_, PaymentInitiationRecipientCreateRequest> {
    ///Set the value of the address field.
    pub fn address(mut self, address: PaymentInitiationAddress) -> Self {
        self.params.address = Some(address);
        self
    }
    ///Set the value of the bacs field.
    pub fn bacs(mut self, bacs: RecipientBacsNullable) -> Self {
        self.params.bacs = Some(bacs);
        self
    }
    ///Set the value of the iban field.
    pub fn iban(mut self, iban: &str) -> Self {
        self.params.iban = Some(iban.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PaymentInitiationRecipientCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::PaymentInitiationRecipientCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/payment_initiation/recipient/create";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.address {
                r = r.json(serde_json::json!({ "address" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.bacs {
                r = r.json(serde_json::json!({ "bacs" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.iban {
                r = r.json(serde_json::json!({ "iban" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "name" : self.params.name }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create payment recipient

Create a payment recipient for payment initiation.  The recipient must be in Europe, within a country that is a member of the Single Euro Payment Area (SEPA) or a non-Eurozone country [supported](https://plaid.com/global) by Plaid. For a standing order (recurring) payment, the recipient must be in the UK.

It is recommended to use `bacs` in the UK and `iban` in EU.

The endpoint is idempotent: if a developer has already made a request with the same payment details, Plaid will return the same `recipient_id`.


See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationrecipientcreate>.*/
    pub fn payment_initiation_recipient_create(
        &self,
        name: &str,
    ) -> FluentRequest<'_, PaymentInitiationRecipientCreateRequest> {
        FluentRequest {
            client: self,
            params: PaymentInitiationRecipientCreateRequest {
                address: None,
                bacs: None,
                iban: None,
                name: name.to_owned(),
            },
        }
    }
}
