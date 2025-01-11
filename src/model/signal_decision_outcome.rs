use serde::{Serialize, Deserialize};
/**The payment decision from the risk assessment.

`APPROVE`: approve the transaction without requiring further actions from your customers. For example, use this field if you are placing a standard hold for all the approved transactions before making funds available to your customers. You should also use this field if you decide to accelerate the fund availability for your customers.

`REVIEW`: the transaction requires manual review

`REJECT`: reject the transaction

`TAKE_OTHER_RISK_MEASURES`: for example, placing a longer hold on funds than those approved transactions or introducing customer frictions such as step-up verification/authentication

`NOT_EVALUATED`: if only logging the Signal results without using them

Possible values:  `APPROVE`, `REVIEW`, `REJECT`, `TAKE_OTHER_RISK_MEASURES`, `NOT_EVALUATED`*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SignalDecisionOutcome {
    #[serde(rename = "APPROVE")]
    Approve,
    #[serde(rename = "REVIEW")]
    Review,
    #[serde(rename = "REJECT")]
    Reject,
    #[serde(rename = "TAKE_OTHER_RISK_MEASURES")]
    TakeOtherRiskMeasures,
    #[serde(rename = "NOT_EVALUATED")]
    NotEvaluated,
}
