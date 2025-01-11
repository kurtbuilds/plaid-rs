use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::IncomeVerificationDocParsingConfig;
/**You should use this struct via [`PlaidClient::credit_payroll_income_parsing_config_update`].

On request success, this will return a [`CreditPayrollIncomeParsingConfigUpdateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditPayrollIncomeParsingConfigUpdateRequest {
    pub item_id: Option<String>,
    pub parsing_config: Vec<IncomeVerificationDocParsingConfig>,
    pub user_token: String,
}
impl FluentRequest<'_, CreditPayrollIncomeParsingConfigUpdateRequest> {
    ///Set the value of the item_id field.
    pub fn item_id(mut self, item_id: &str) -> Self {
        self.params.item_id = Some(item_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreditPayrollIncomeParsingConfigUpdateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::CreditPayrollIncomeParsingConfigUpdateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/payroll_income/parsing_config/update";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.item_id {
                r = r.json(serde_json::json!({ "item_id" : unwrapped }));
            }
            r = r
                .json(
                    serde_json::json!({ "parsing_config" : self.params.parsing_config }),
                );
            r = r.json(serde_json::json!({ "user_token" : self.params.user_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Update the parsing configuration for a document income verification

`/credit/payroll_income/parsing_config/update` updates the parsing configuration for a document income verification.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditpayroll_incomeparsing_configupdate>.*/
    pub fn credit_payroll_income_parsing_config_update(
        &self,
        parsing_config: Vec<IncomeVerificationDocParsingConfig>,
        user_token: &str,
    ) -> FluentRequest<'_, CreditPayrollIncomeParsingConfigUpdateRequest> {
        FluentRequest {
            client: self,
            params: CreditPayrollIncomeParsingConfigUpdateRequest {
                item_id: None,
                parsing_config,
                user_token: user_token.to_owned(),
            },
        }
    }
}
