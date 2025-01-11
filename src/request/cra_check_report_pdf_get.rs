use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::CraPDFAddOns;
/**You should use this struct via [`PlaidClient::cra_check_report_pdf_get`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraCheckReportPdfGetRequest {
    pub add_ons: Option<Vec<CraPdfAddOns>>,
    pub third_party_user_token: Option<String>,
    pub user_token: Option<String>,
}
impl FluentRequest<'_, CraCheckReportPdfGetRequest> {
    ///Set the value of the add_ons field.
    pub fn add_ons(mut self, add_ons: Vec<CraPdfAddOns>) -> Self {
        self.params.add_ons = Some(add_ons);
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
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CraCheckReportPdfGetRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/cra/check_report/pdf/get";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.add_ons {
                r = r.json(serde_json::json!({ "add_ons" : unwrapped }));
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
    /**Retrieve Consumer Reports as a PDF

`/cra/check_report/pdf/get` retrieves the most recent Consumer Report in PDF format. By default, the most recent Base Report (if it exists) for the user will be returned. To request that the most recent Income Insights report be included in the PDF as well, use the `add-ons` field.

See endpoint docs at <https://plaid.com/docs/check/api/#cracheck_reportpdfget>.*/
    pub fn cra_check_report_pdf_get(
        &self,
    ) -> FluentRequest<'_, CraCheckReportPdfGetRequest> {
        FluentRequest {
            client: self,
            params: CraCheckReportPdfGetRequest {
                add_ons: None,
                third_party_user_token: None,
                user_token: None,
            },
        }
    }
}
