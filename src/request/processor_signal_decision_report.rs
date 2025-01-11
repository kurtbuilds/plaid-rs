use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{SignalDecisionOutcome, SignalPaymentMethod};
/**You should use this struct via [`PlaidClient::processor_signal_decision_report`].

On request success, this will return a [`ProcessorSignalDecisionReportResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorSignalDecisionReportRequest {
    pub amount_instantly_available: Option<f64>,
    pub client_transaction_id: String,
    pub days_funds_on_hold: Option<i64>,
    pub decision_outcome: Option<SignalDecisionOutcome>,
    pub initiated: bool,
    pub payment_method: Option<SignalPaymentMethod>,
    pub processor_token: String,
}
impl FluentRequest<'_, ProcessorSignalDecisionReportRequest> {
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
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProcessorSignalDecisionReportRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::ProcessorSignalDecisionReportResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/signal/decision/report";
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
            r = r
                .json(
                    serde_json::json!(
                        { "processor_token" : self.params.processor_token }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Report whether you initiated an ACH transaction

After calling `/processor/signal/evaluate`, call `/processor/signal/decision/report` to report whether the transaction was initiated.

If you are using the [Plaid Transfer product](https://www.plaid.com/docs/transfer) to create transfers, it is not necessary to use this endpoint, as Plaid already knows whether the transfer was initiated.

See endpoint docs at <https://plaid.com/docs/api/processor-partners/#processorsignaldecisionreport>.*/
    pub fn processor_signal_decision_report(
        &self,
        client_transaction_id: &str,
        initiated: bool,
        processor_token: &str,
    ) -> FluentRequest<'_, ProcessorSignalDecisionReportRequest> {
        FluentRequest {
            client: self,
            params: ProcessorSignalDecisionReportRequest {
                amount_instantly_available: None,
                client_transaction_id: client_transaction_id.to_owned(),
                days_funds_on_hold: None,
                decision_outcome: None,
                initiated,
                payment_method: None,
                processor_token: processor_token.to_owned(),
            },
        }
    }
}
