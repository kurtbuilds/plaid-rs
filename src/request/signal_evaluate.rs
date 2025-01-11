use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{SignalDevice, SignalUser};
/**You should use this struct via [`PlaidClient::signal_evaluate`].

On request success, this will return a [`SignalEvaluateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalEvaluateRequest {
    pub access_token: String,
    pub account_id: String,
    pub amount: f64,
    pub client_transaction_id: String,
    pub client_user_id: Option<String>,
    pub default_payment_method: Option<String>,
    pub device: Option<SignalDevice>,
    pub is_recurring: Option<bool>,
    pub risk_profile_key: Option<String>,
    pub ruleset_key: Option<String>,
    pub user: Option<SignalUser>,
    pub user_present: Option<bool>,
}
pub struct SignalEvaluateRequired<'a> {
    pub access_token: &'a str,
    pub account_id: &'a str,
    pub amount: f64,
    pub client_transaction_id: &'a str,
}
impl FluentRequest<'_, SignalEvaluateRequest> {
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
    ///Set the value of the risk_profile_key field.
    pub fn risk_profile_key(mut self, risk_profile_key: &str) -> Self {
        self.params.risk_profile_key = Some(risk_profile_key.to_owned());
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
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, SignalEvaluateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::SignalEvaluateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/signal/evaluate";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = r.json(serde_json::json!({ "account_id" : self.params.account_id }));
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
            if let Some(ref unwrapped) = self.params.risk_profile_key {
                r = r.json(serde_json::json!({ "risk_profile_key" : unwrapped }));
            }
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

Use `/signal/evaluate` to evaluate a planned ACH transaction to get a return risk assessment (such as a risk score and risk tier) and additional risk signals.

In order to obtain a valid score for an ACH transaction, Plaid must have an access token for the account, and the Item must be healthy (receiving product updates) or have recently been in a healthy state. If the transaction does not meet eligibility requirements, an error will be returned corresponding to the underlying cause. If `/signal/evaluate` is called on the same transaction multiple times within a 24-hour period, cached results may be returned. For more information please refer to the error documentation on [Item errors](/docs/errors/item/) and [Link in Update Mode](/docs/link/update-mode/).

Note: This request may take some time to complete if Signal is being added to an existing Item. This is because Plaid must communicate directly with the institution when retrieving the data for the first time.

See endpoint docs at <https://plaid.com/docs/api/products/signal#signalevaluate>.*/
    pub fn signal_evaluate(
        &self,
        args: SignalEvaluateRequired,
    ) -> FluentRequest<'_, SignalEvaluateRequest> {
        FluentRequest {
            client: self,
            params: SignalEvaluateRequest {
                access_token: args.access_token.to_owned(),
                account_id: args.account_id.to_owned(),
                amount: args.amount,
                client_transaction_id: args.client_transaction_id.to_owned(),
                client_user_id: None,
                default_payment_method: None,
                device: None,
                is_recurring: None,
                risk_profile_key: None,
                ruleset_key: None,
                user: None,
                user_present: None,
            },
        }
    }
}
