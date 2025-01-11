use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::cra_check_report_base_report_get`].

On request success, this will return a [`CraCheckReportBaseReportGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraCheckReportBaseReportGetRequest {
    pub item_ids: Option<Vec<String>>,
    pub third_party_user_token: Option<String>,
    pub user_token: Option<String>,
}
impl FluentRequest<'_, CraCheckReportBaseReportGetRequest> {
    ///Set the value of the item_ids field.
    pub fn item_ids(
        mut self,
        item_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .item_ids = Some(
            item_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    ///Set the value of the third_party_user_token field.
    pub fn third_party_user_token(mut self, third_party_user_token: &str) -> Self {
        self.params.third_party_user_token = Some(third_party_user_token.to_owned());
        self
    }
    ///Set the value of the user_token field.
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.params.user_token = Some(user_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CraCheckReportBaseReportGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::CraCheckReportBaseReportGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/cra/check_report/base_report/get";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.item_ids {
                r = r.json(serde_json::json!({ "item_ids" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.third_party_user_token {
                r = r.json(serde_json::json!({ "third_party_user_token" : unwrapped }));
            }
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
    /**Retrieve a Base Report

This endpoint allows you to retrieve the Base Report for your user, allowing you to receive comprehensive bank account and cash flow data. You should call this endpoint after you've received a `CHECK_REPORT_READY` webhook, either after the Link session for the user or after calling `/cra/check_report/create`. If the most recent consumer report for the user doesn't have sufficient data to generate the base report, or the consumer report has expired, you will receive an error indicating that you should create a new consumer report by calling `/cra/check_report/create`.

See endpoint docs at <https://plaid.com/docs/check/api/#cracheck_reportbase_reportget>.*/
    pub fn cra_check_report_base_report_get(
        &self,
    ) -> FluentRequest<'_, CraCheckReportBaseReportGetRequest> {
        FluentRequest {
            client: self,
            params: CraCheckReportBaseReportGetRequest {
                item_ids: None,
                third_party_user_token: None,
                user_token: None,
            },
        }
    }
}
