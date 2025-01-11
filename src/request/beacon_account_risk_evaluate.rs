use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{
    SignalDevice, BeaconAccountRiskEvaluateEvaluationReason,
    BeaconAccountRiskEvaluateRequestOptions,
};
/**You should use this struct via [`PlaidClient::beacon_account_risk_evaluate`].

On request success, this will return a [`BeaconAccountRiskEvaluateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconAccountRiskEvaluateRequest {
    pub access_token: Option<String>,
    pub client_evaluation_id: Option<String>,
    pub client_user_id: Option<String>,
    pub device: Option<SignalDevice>,
    pub evaluate_time: Option<String>,
    pub evaluation_reason: Option<BeaconAccountRiskEvaluateEvaluationReason>,
    pub options: Option<BeaconAccountRiskEvaluateRequestOptions>,
}
impl FluentRequest<'_, BeaconAccountRiskEvaluateRequest> {
    ///Set the value of the access_token field.
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.params.access_token = Some(access_token.to_owned());
        self
    }
    ///Set the value of the client_evaluation_id field.
    pub fn client_evaluation_id(mut self, client_evaluation_id: &str) -> Self {
        self.params.client_evaluation_id = Some(client_evaluation_id.to_owned());
        self
    }
    ///Set the value of the client_user_id field.
    pub fn client_user_id(mut self, client_user_id: &str) -> Self {
        self.params.client_user_id = Some(client_user_id.to_owned());
        self
    }
    ///Set the value of the device field.
    pub fn device(mut self, device: SignalDevice) -> Self {
        self.params.device = Some(device);
        self
    }
    ///Set the value of the evaluate_time field.
    pub fn evaluate_time(mut self, evaluate_time: &str) -> Self {
        self.params.evaluate_time = Some(evaluate_time.to_owned());
        self
    }
    ///Set the value of the evaluation_reason field.
    pub fn evaluation_reason(
        mut self,
        evaluation_reason: BeaconAccountRiskEvaluateEvaluationReason,
    ) -> Self {
        self.params.evaluation_reason = Some(evaluation_reason);
        self
    }
    ///Set the value of the options field.
    pub fn options(mut self, options: BeaconAccountRiskEvaluateRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, BeaconAccountRiskEvaluateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::BeaconAccountRiskEvaluateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beacon/account_risk/v1/evaluate";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.access_token {
                r = r.json(serde_json::json!({ "access_token" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.client_evaluation_id {
                r = r.json(serde_json::json!({ "client_evaluation_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.client_user_id {
                r = r.json(serde_json::json!({ "client_user_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.device {
                r = r.json(serde_json::json!({ "device" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.evaluate_time {
                r = r.json(serde_json::json!({ "evaluate_time" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.evaluation_reason {
                r = r.json(serde_json::json!({ "evaluation_reason" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Evaluate risk of a bank account

Use `/beacon/account_risk/v1/evaluate` to get risk insights for a linked account.

See endpoint docs at <https://plaid.com/docsnone>.*/
    pub fn beacon_account_risk_evaluate(
        &self,
    ) -> FluentRequest<'_, BeaconAccountRiskEvaluateRequest> {
        FluentRequest {
            client: self,
            params: BeaconAccountRiskEvaluateRequest {
                access_token: None,
                client_evaluation_id: None,
                client_user_id: None,
                device: None,
                evaluate_time: None,
                evaluation_reason: None,
                options: None,
            },
        }
    }
}
