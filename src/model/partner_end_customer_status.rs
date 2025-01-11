use serde::{Serialize, Deserialize};
/**The status of the given end customer.

`UNDER_REVIEW`: The end customer has been created and enabled in Sandbox and Limited Production. The end customer must be manually reviewed by the Plaid team before it can be enabled in full production, at which point its status will automatically transition to `PENDING_ENABLEMENT` or `DENIED`.

`PENDING_ENABLEMENT`: The end customer is ready to be fully enabled in the Production environment. Call the `/partner/customer/enable` endpoint to enable the end customer in full Production.

`ACTIVE`: The end customer has been fully enabled in all environments.

`DENIED`: The end customer has been created and enabled in Sandbox and Limited Production, but it did not pass review by the Plaid team and therefore cannot be enabled for full Production access. Talk to your Account Manager for more information.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PartnerEndCustomerStatus {
    #[serde(rename = "UNDER_REVIEW")]
    UnderReview,
    #[serde(rename = "PENDING_ENABLEMENT")]
    PendingEnablement,
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "DENIED")]
    Denied,
}
