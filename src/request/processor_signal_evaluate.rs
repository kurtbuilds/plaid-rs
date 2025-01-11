use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{SignalDevice, SignalUser};
/**You should use this struct via [`PlaidClient::processor_signal_evaluate`].

On request success, this will return a [`ProcessorSignalEvaluateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorSignalEvaluateRequest {
    pub amount: f64,
    pub client_transaction_id: String,
    pub client_user_id: Option<String>,
    pub default_payment_method: Option<String>,
    pub device: Option<SignalDevice>,
    pub is_recurring: Option<bool>,
    pub processor_token: String,
    pub ruleset_key: Option<String>,
    pub user: Option<SignalUser>,
    pub user_present: Option<bool>,
}
impl FluentRequest<'_, ProcessorSignalEvaluateRequest> {
    ///Set the value of the client_user_id field.
    pub fn client_user_id(mut self, client_user_id: &str) -> Self {
        self.params.client_user_id = Some(client_user_id.to_owned());
        self
    }
    ///Set the value of the default_payment_method field.
    pub fn default_payment_method(mut self, default_payment_method: &str) -> Self {
        self.params.default_payment_method = Some(default_payment_method.to_owned());
        self
    }
    ///Set the value of the device field.
    pub fn device(mut self, device: SignalDevice) -> Self {
        self.params.device = Some(device);
        self
    }
    ///Set the value of the is_recurring field.
    pub fn is_recurring(mut self, is_recurring: bool) -> Self {
        self.params.is_recurring = Some(is_recurring);
        self
    }
    ///Set the value of the ruleset_key field.
    pub fn ruleset_key(mut self, ruleset_key: &str) -> Self {
        self.params.ruleset_key = Some(ruleset_key.to_owned());
        self
    }
    ///Set the value of the user field.
    pub fn user(mut self, user: SignalUser) -> Self {
        self.params.user = Some(user);
        self
    }
    ///Set the value of the user_present field.
    pub fn user_present(mut self, user_present: bool) -> Self {
        self.params.user_present = Some(user_present);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProcessorSignalEvaluateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::ProcessorSignalEvaluateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/signal/evaluate";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "amount" : self.params.amount }));
            r = r
                .json(
                    serde_json::json!(
                        { "client_transaction_id" : self.params.client_transaction_id }
                    ),
                );
            if let Some(ref unwrapped) = self.params.client_user_id {
                r = r.json(serde_json::json!({ "client_user_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.default_payment_method {
                r = r.json(serde_json::json!({ "default_payment_method" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.device {
                r = r.json(serde_json::json!({ "device" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.is_recurring {
                r = r.json(serde_json::json!({ "is_recurring" : unwrapped }));
            }
            r = r
                .json(
                    serde_json::json!(
                        { "processor_token" : self.params.processor_token }
                    ),
                );
            if let Some(ref unwrapped) = self.params.ruleset_key {
                r = r.json(serde_json::json!({ "ruleset_key" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.user {
                r = r.json(serde_json::json!({ "user" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.user_present {
                r = r.json(serde_json::json!({ "user_present" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Evaluate a planned ACH transaction

Use `/processor/signal/evaluate` to evaluate a planned ACH transaction as a processor to get a return risk assessment (such as a risk score and risk tier) and additional risk signals.

In order to obtain a valid score for an ACH transaction, Plaid must have an access token for the account, and the Item must be healthy (receiving product updates) or have recently been in a healthy state. If the transaction does not meet eligibility requirements, an error will be returned corresponding to the underlying cause. If `/processor/signal/evaluate` is called on the same transaction multiple times within a 24-hour period, cached results may be returned. For more information please refer to our error documentation on [item errors](/docs/errors/item/) and [Link in Update Mode](/docs/link/update-mode/).

Note: This request may take some time to complete if Signal is being added to an existing Item. This is because Plaid must communicate directly with the institution when retrieving the data for the first time. To reduce this latency, you can call `/signal/prepare` on the Item before you need to request Signal data.

See endpoint docs at <https://plaid.com/docs/api/processor-partners/#processorsignalevaluate>.*/
    pub fn processor_signal_evaluate(
        &self,
        amount: f64,
        client_transaction_id: &str,
        processor_token: &str,
    ) -> FluentRequest<'_, ProcessorSignalEvaluateRequest> {
        FluentRequest {
            client: self,
            params: ProcessorSignalEvaluateRequest {
                amount,
                client_transaction_id: client_transaction_id.to_owned(),
                client_user_id: None,
                default_payment_method: None,
                device: None,
                is_recurring: None,
                processor_token: processor_token.to_owned(),
                ruleset_key: None,
                user: None,
                user_present: None,
            },
        }
    }
}
