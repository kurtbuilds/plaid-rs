use serde::{Serialize, Deserialize};
use super::{AssetReportAddOns, AssetReportUser};
///An optional object to filter `/asset_report/create` results. If provided, must be non-`null`. The optional `user` object is required for the report to be eligible for Fannie Mae's Day 1 Certainty program.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportCreateRequestOptions {
    ///This field can be used to add additional options for the Asset Report. To fetch `investments` data (transactions, holdings, etc.) in the Asset Report, `investments` must be specified in `add_ons`. For Fast Assets, `fast_assets` must be specified in `add_ons`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_ons: Option<Vec<AssetReportAddOns>>,
    ///Client-generated identifier, which can be used by lenders to track loan applications.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_report_id: Option<String>,
    ///true to return balance and identity earlier as a fast report. Defaults to false if omitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_fast_report: Option<bool>,
    ///Additional information that can be included in the asset report. Possible values: `"investments"`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<String>>,
    ///If set to false, only 1 item must be healthy at the time of report creation. The default value is true, which would require all items to be healthy at the time of report creation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_all_items: Option<bool>,
    ///The user object allows you to provide additional information about the user to be appended to the Asset Report. All fields are optional. The `first_name`, `last_name`, and `ssn` fields are required if you would like the Report to be eligible for Fannie Mae’s Day 1 Certainty™ program.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<AssetReportUser>,
    ///URL to which Plaid will send Assets webhooks, for example when the requested Asset Report is ready.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
}
impl std::fmt::Display for AssetReportCreateRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
