use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::credit_payroll_income_risk_signals_get`].

On request success, this will return a [`CreditPayrollIncomeRiskSignalsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditPayrollIncomeRiskSignalsGetRequest {
    pub user_token: Option<String>,
}
impl FluentRequest<'_, CreditPayrollIncomeRiskSignalsGetRequest> {
    ///Set the value of the user_token field.
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.params.user_token = Some(user_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreditPayrollIncomeRiskSignalsGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::CreditPayrollIncomeRiskSignalsGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/payroll_income/risk_signals/get";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.user_token {
                r = r.json(serde_json::json!({ "user_token" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve fraud insights for a user's manually uploaded document(s).

`/credit/payroll_income/risk_signals/get` can be used as part of the Document Income flow to assess a user-uploaded document for signs of potential fraud or tampering. It returns a risk score for each uploaded document that indicates the likelihood of the document being fraudulent, in addition to details on the individual risk signals contributing to the score.

To trigger risk signal generation for an Item, call `/link/token/create` with `parsing_config` set to include `risk_signals`, or call `/credit/payroll_income/parsing_config/update`. Once risk signal generation has been triggered, `/credit/payroll_income/risk_signals/get` can be called at any time after the `INCOME_VERIFICATION_RISK_SIGNALS` webhook has been fired.

`/credit/payroll_income/risk_signals/get` is offered as an add-on to Document Income and is billed separately. To request access to this endpoint, submit a product access request or contact your Plaid account manager.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditpayroll_incomerisk_signalsget>.*/
    pub fn credit_payroll_income_risk_signals_get(
        &self,
    ) -> FluentRequest<'_, CreditPayrollIncomeRiskSignalsGetRequest> {
        FluentRequest {
            client: self,
            params: CreditPayrollIncomeRiskSignalsGetRequest {
                user_token: None,
            },
        }
    }
}
