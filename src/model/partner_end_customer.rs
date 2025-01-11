use serde::{Serialize, Deserialize};
use super::PartnerEndCustomerStatus;
///The details for an end customer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerEndCustomer {
    ///The company name associated with the end customer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /**The status of the given end customer.

`UNDER_REVIEW`: The end customer has been created and enabled in Sandbox and Limited Production. The end customer must be manually reviewed by the Plaid team before it can be enabled in full production, at which point its status will automatically transition to `PENDING_ENABLEMENT` or `DENIED`.

`PENDING_ENABLEMENT`: The end customer is ready to be fully enabled in the Production environment. Call the `/partner/customer/enable` endpoint to enable the end customer in full Production.

`ACTIVE`: The end customer has been fully enabled in all environments.

`DENIED`: The end customer has been created and enabled in Sandbox and Limited Production, but it did not pass review by the Plaid team and therefore cannot be enabled for full Production access. Talk to your Account Manager for more information.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PartnerEndCustomerStatus>,
}
impl std::fmt::Display for PartnerEndCustomer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
