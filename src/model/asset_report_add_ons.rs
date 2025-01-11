use serde::{Serialize, Deserialize};
/**A list of add-ons that should be included in the Asset Report.

`fast_assets`: When Fast Assets is requested, Plaid will create two versions of the Asset Report: the Fast Asset Report, which will contain only Identity and Balance information, and the Full Asset Report, which will also contain Transactions information. A `PRODUCT_READY` webhook will be fired for each Asset Report when it is ready, and the `report_type` field will indicate whether the webhook is firing for the `full` or `fast` Asset Report. To retrieve the Fast Asset Report, call `/asset_report/get` with `fast_report` set to `true`. There is no additional charge for using Fast Assets. To create a Fast Asset Report, Plaid must successfully retrieve both Identity and Balance data; if Plaid encounters an error obtaining this data, the Fast Asset Report will not be created. However, as long as Plaid can obtain Transactions data, the Full Asset Report will still be available.

`investments`: Request an Asset Report with Investments. This add-on is in closed beta and not generally available.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AssetReportAddOns {
    #[serde(rename = "investments")]
    Investments,
    #[serde(rename = "fast_assets")]
    FastAssets,
}
