use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{PaymentAmountToRefund, PaymentInitiationAddress};
/**You should use this struct via [`PlaidClient::payment_initiation_payment_reverse`].

On request success, this will return a [`PaymentInitiationPaymentReverseResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentReverseRequest {
    pub amount: Option<PaymentAmountToRefund>,
    pub counterparty_address: Option<PaymentInitiationAddress>,
    pub counterparty_date_of_birth: Option<chrono::NaiveDate>,
    pub idempotency_key: String,
    pub payment_id: String,
    pub reference: String,
}
impl FluentRequest<'_, PaymentInitiationPaymentReverseRequest> {
    ///Set the value of the amount field.
    pub fn amount(mut self, amount: PaymentAmountToRefund) -> Self {
        self.params.amount = Some(amount);
        self
    }
    ///Set the value of the counterparty_address field.
    pub fn counterparty_address(
        mut self,
        counterparty_address: PaymentInitiationAddress,
    ) -> Self {
        self.params.counterparty_address = Some(counterparty_address);
        self
    }
    ///Set the value of the counterparty_date_of_birth field.
    pub fn counterparty_date_of_birth(
        mut self,
        counterparty_date_of_birth: chrono::NaiveDate,
    ) -> Self {
        self.params.counterparty_date_of_birth = Some(counterparty_date_of_birth);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PaymentInitiationPaymentReverseRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::PaymentInitiationPaymentReverseResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/payment_initiation/payment/reverse";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.amount {
                r = r.json(serde_json::json!({ "amount" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.counterparty_address {
                r = r.json(serde_json::json!({ "counterparty_address" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.counterparty_date_of_birth {
                r = r
                    .json(
                        serde_json::json!({ "counterparty_date_of_birth" : unwrapped }),
                    );
            }
            r = r
                .json(
                    serde_json::json!(
                        { "idempotency_key" : self.params.idempotency_key }
                    ),
                );
            r = r.json(serde_json::json!({ "payment_id" : self.params.payment_id }));
            r = r.json(serde_json::json!({ "reference" : self.params.reference }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Reverse an existing payment

Reverse a settled payment from a Plaid virtual account.

The original payment must be in a settled state to be refunded.
To refund partially, specify the amount as part of the request.
If the amount is not specified, the refund amount will be equal to all
of the remaining payment amount that has not been refunded yet.

The refund will go back to the source account that initiated the payment.
The original payment must have been initiated to a Plaid virtual account
so that this account can be used to initiate the refund.

Providing counterparty information such as date of birth and address increases
the likelihood of refund being successful without human intervention.


See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationpaymentreverse>.*/
    pub fn payment_initiation_payment_reverse(
        &self,
        idempotency_key: &str,
        payment_id: &str,
        reference: &str,
    ) -> FluentRequest<'_, PaymentInitiationPaymentReverseRequest> {
        FluentRequest {
            client: self,
            params: PaymentInitiationPaymentReverseRequest {
                amount: None,
                counterparty_address: None,
                counterparty_date_of_birth: None,
                idempotency_key: idempotency_key.to_owned(),
                payment_id: payment_id.to_owned(),
                reference: reference.to_owned(),
            },
        }
    }
}
