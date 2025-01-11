use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{SignalDecisionOutcome, SignalPaymentMethod};
/**You should use this struct via [`PlaidClient::signal_decision_report`].

On request success, this will return a [`SignalDecisionReportResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalDecisionReportRequest {
    pub amount_instantly_available: Option<f64>,
    pub client_transaction_id: String,
    pub days_funds_on_hold: Option<i64>,
    pub decision_outcome: Option<SignalDecisionOutcome>,
    pub initiated: bool,
    pub payment_method: Option<SignalPaymentMethod>,
}
impl FluentRequest<'_, SignalDecisionReportRequest> {
    ///Set the value of the amount_instantly_available field.
    pub fn amount_instantly_available(
        mut self,
        amount_instantly_available: f64,
    ) -> Self {
        self.params.amount_instantly_available = Some(amount_instantly_available);
        self
    }
    ///Set the value of the days_funds_on_hold field.
    pub fn days_funds_on_hold(mut self, days_funds_on_hold: i64) -> Self {
        self.params.days_funds_on_hold = Some(days_funds_on_hold);
        self
    }
    ///Set the value of the decision_outcome field.
    pub fn decision_outcome(mut self, decision_outcome: SignalDecisionOutcome) -> Self {
        self.params.decision_outcome = Some(decision_outcome);
        self
    }
    ///Set the value of the payment_method field.
    pub fn payment_method(mut self, payment_method: SignalPaymentMethod) -> Self {
        self.params.payment_method = Some(payment_method);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, SignalDecisionReportRequest> {
    type Output = httpclient::InMemoryResult<crate::model::SignalDecisionReportResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/signal/decision/report";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.amount_instantly_available {
                r = r
                    .json(
                        serde_json::json!({ "amount_instantly_available" : unwrapped }),
                    );
            }
            r = r
                .json(
                    serde_json::json!(
                        { "client_transaction_id" : self.params.client_transaction_id }
                    ),
                );
            if let Some(ref unwrapped) = self.params.days_funds_on_hold {
                r = r.json(serde_json::json!({ "days_funds_on_hold" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.decision_outcome {
                r = r.json(serde_json::json!({ "decision_outcome" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "initiated" : self.params.initiated }));
            if let Some(ref unwrapped) = self.params.payment_method {
                r = r.json(serde_json::json!({ "payment_method" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Report whether you initiated an ACH transaction

After calling `/signal/evaluate`, call `/signal/decision/report` to report whether the transaction was initiated.

See endpoint docs at <https://plaid.com/docs/api/products/signal#signaldecisionreport>.*/
    pub fn signal_decision_report(
        &self,
        client_transaction_id: &str,
        initiated: bool,
    ) -> FluentRequest<'_, SignalDecisionReportRequest> {
        FluentRequest {
            client: self,
            params: SignalDecisionReportRequest {
                amount_instantly_available: None,
                client_transaction_id: client_transaction_id.to_owned(),
                days_funds_on_hold: None,
                decision_outcome: None,
                initiated,
                payment_method: None,
            },
        }
    }
}
