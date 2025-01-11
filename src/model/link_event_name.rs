use serde::{Serialize, Deserialize};
///A string representing the event that has just occurred in the Link flow.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LinkEventName {
    #[serde(rename = "BANK_INCOME_INSIGHTS_COMPLETED")]
    BankIncomeInsightsCompleted,
    #[serde(rename = "CLOSE_OAUTH")]
    CloseOauth,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "EXIT")]
    Exit,
    #[serde(rename = "FAIL_OAUTH")]
    FailOauth,
    #[serde(rename = "HANDOFF")]
    Handoff,
    #[serde(rename = "ISSUE_FOLLOWED")]
    IssueFollowed,
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "OPEN_MY_PLAID")]
    OpenMyPlaid,
    #[serde(rename = "OPEN_OAUTH")]
    OpenOauth,
    #[serde(rename = "SEARCH_INSTITUTION")]
    SearchInstitution,
    #[serde(rename = "SELECT_AUTH_TYPE")]
    SelectAuthType,
    #[serde(rename = "SELECT_BRAND")]
    SelectBrand,
    #[serde(rename = "SELECT_DEGRADED_INSTITUTION")]
    SelectDegradedInstitution,
    #[serde(rename = "SELECT_DOWN_INSTITUTION")]
    SelectDownInstitution,
    #[serde(rename = "SELECT_FILTERED_INSTITUTION")]
    SelectFilteredInstitution,
    #[serde(rename = "SELECT_INSTITUTION")]
    SelectInstitution,
    #[serde(rename = "SUBMIT_ACCOUNT_NUMBER")]
    SubmitAccountNumber,
    #[serde(rename = "SUBMIT_CREDENTIALS")]
    SubmitCredentials,
    #[serde(rename = "SUBMIT_DOCUMENTS")]
    SubmitDocuments,
    #[serde(rename = "SUBMIT_DOCUMENTS_ERROR")]
    SubmitDocumentsError,
    #[serde(rename = "SUBMIT_DOCUMENTS_SUCCESS")]
    SubmitDocumentsSuccess,
    #[serde(rename = "SUBMIT_MFA")]
    SubmitMfa,
    #[serde(rename = "SUBMIT_ROUTING_NUMBER")]
    SubmitRoutingNumber,
    #[serde(rename = "TRANSITION_VIEW")]
    TransitionView,
    #[serde(rename = "VIEW_DATA_TYPES")]
    ViewDataTypes,
}
