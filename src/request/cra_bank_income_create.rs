use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::ConsumerReportPermissiblePurpose;
/**You should use this struct via [`PlaidClient::cra_bank_income_create`].

On request success, this will return a [`CraBankIncomeCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraBankIncomeCreateRequest {
    pub consumer_report_permissible_purpose: Option<ConsumerReportPermissiblePurpose>,
    pub days_requested: Option<i64>,
    pub user_token: Option<String>,
    pub webhook: Option<String>,
}
impl FluentRequest<'_, CraBankIncomeCreateRequest> {
    ///Set the value of the consumer_report_permissible_purpose field.
    pub fn consumer_report_permissible_purpose(
        mut self,
        consumer_report_permissible_purpose: ConsumerReportPermissiblePurpose,
    ) -> Self {
        self
            .params
            .consumer_report_permissible_purpose = Some(
            consumer_report_permissible_purpose,
        );
        self
    }
    ///Set the value of the days_requested field.
    pub fn days_requested(mut self, days_requested: i64) -> Self {
        self.params.days_requested = Some(days_requested);
        self
    }
    ///Set the value of the user_token field.
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.params.user_token = Some(user_token.to_owned());
        self
    }
    ///Set the value of the webhook field.
    pub fn webhook(mut self, webhook: &str) -> Self {
        self.params.webhook = Some(webhook.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CraBankIncomeCreateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::CraBankIncomeCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/cra/bank_income/create";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.consumer_report_permissible_purpose
            {
                r = r
                    .json(
                        serde_json::json!(
                            { "consumer_report_permissible_purpose" : unwrapped }
                        ),
                    );
            }
            if let Some(ref unwrapped) = self.params.days_requested {
                r = r.json(serde_json::json!({ "days_requested" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.user_token {
                r = r.json(serde_json::json!({ "user_token" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.webhook {
                r = r.json(serde_json::json!({ "webhook" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create a CRA report for income verification

`/cra/bank_income/create` creates a CRA report for income verification

See endpoint docs at <https://plaid.com/docs/api/products/income/#crabank_incomecreate>.*/
    pub fn cra_bank_income_create(
        &self,
    ) -> FluentRequest<'_, CraBankIncomeCreateRequest> {
        FluentRequest {
            client: self,
            params: CraBankIncomeCreateRequest {
                consumer_report_permissible_purpose: None,
                days_requested: None,
                user_token: None,
                webhook: None,
            },
        }
    }
}
