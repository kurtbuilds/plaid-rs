use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{FraudAmount, BeaconReportCreateType};
/**You should use this struct via [`PlaidClient::beacon_report_create`].

On request success, this will return a [`BeaconReportCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconReportCreateRequest {
    pub beacon_user_id: String,
    pub fraud_amount: Option<FraudAmount>,
    pub fraud_date: chrono::NaiveDate,
    pub type_: BeaconReportCreateType,
}
impl FluentRequest<'_, BeaconReportCreateRequest> {
    ///Set the value of the fraud_amount field.
    pub fn fraud_amount(mut self, fraud_amount: FraudAmount) -> Self {
        self.params.fraud_amount = Some(fraud_amount);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BeaconReportCreateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::BeaconReportCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beacon/report/create";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!({ "beacon_user_id" : self.params.beacon_user_id }),
                );
            if let Some(ref unwrapped) = self.params.fraud_amount {
                r = r.json(serde_json::json!({ "fraud_amount" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "fraud_date" : self.params.fraud_date }));
            r = r.json(serde_json::json!({ "type" : self.params.type_ }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create a Beacon Report

Create a fraud report for a given Beacon User.

See endpoint docs at <https://plaid.com/docs/api/products/beacon/#beaconreportcreate>.*/
    pub fn beacon_report_create(
        &self,
        beacon_user_id: &str,
        fraud_date: chrono::NaiveDate,
        type_: BeaconReportCreateType,
    ) -> FluentRequest<'_, BeaconReportCreateRequest> {
        FluentRequest {
            client: self,
            params: BeaconReportCreateRequest {
                beacon_user_id: beacon_user_id.to_owned(),
                fraud_amount: None,
                fraud_date,
                type_,
            },
        }
    }
}
