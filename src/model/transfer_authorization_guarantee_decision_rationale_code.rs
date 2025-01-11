use serde::{Serialize, Deserialize};
/**A code representing the reason Plaid declined to guarantee this transfer:

`RETURN_BANK`: The risk of a bank-initiated return (for example, an R01/NSF) is too high to guarantee this transfer.

`RETURN_CUSTOMER`: The risk of a customer-initiated return (for example, a R10/Unauthorized) is too high to guarantee this transfer.

`GUARANTEE_LIMIT_REACHED`: This transfer is low-risk, but Guarantee has exhausted an internal limit on the number or rate of guarantees that applies to this transfer.

`RISK_ESTIMATE_UNAVAILABLE`: A risk estimate is unavailable for this Item.

`REQUIRED_PARAM_MISSING`: Required fields are missing.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferAuthorizationGuaranteeDecisionRationaleCode {
    #[serde(rename = "RETURN_BANK")]
    ReturnBank,
    #[serde(rename = "RETURN_CUSTOMER")]
    ReturnCustomer,
    #[serde(rename = "GUARANTEE_LIMIT_REACHED")]
    GuaranteeLimitReached,
    #[serde(rename = "RISK_ESTIMATE_UNAVAILABLE")]
    RiskEstimateUnavailable,
    #[serde(rename = "REQUIRED_PARAM_MISSING")]
    RequiredParamMissing,
}
