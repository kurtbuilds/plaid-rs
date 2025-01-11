use serde::{Serialize, Deserialize};
/**Description of the reason you want to evaluate risk.
`ONBOARDING`: user links a first bank account as part of the onboarding flow of your platform.
`NEW_ACCOUNT`: user links another bank account or replaces the currently linked bank account on your platform.
`INFORMATION_CHANGE`: user changes their information on your platform, e.g., updating their phone number.
`DORMANT_USER`:  you decide to re-evaluate a user that becomes active after a period of inactivity.
`OTHER`: any other reasons not listed here
Possible values:  `ONBOARDING`, `NEW_ACCOUNT`, `INFORMATION_CHANGE`, `DORMANT_USER`, `OTHER`*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BeaconAccountRiskEvaluateEvaluationReason {
    #[serde(rename = "ONBOARDING")]
    Onboarding,
    #[serde(rename = "NEW_ACCOUNT")]
    NewAccount,
    #[serde(rename = "INFORMATION_CHANGE")]
    InformationChange,
    #[serde(rename = "DORMANT_USER")]
    DormantUser,
    #[serde(rename = "OTHER")]
    Other,
}
