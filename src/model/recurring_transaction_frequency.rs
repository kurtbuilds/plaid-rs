use serde::{Serialize, Deserialize};
/**Describes the frequency of the transaction stream.

`WEEKLY`: Assigned to a transaction stream that occurs approximately every week.

`BIWEEKLY`: Assigned to a transaction stream that occurs approximately every 2 weeks.

`SEMI_MONTHLY`: Assigned to a transaction stream that occurs approximately twice per month. This frequency is typically seen for inflow transaction streams.

`MONTHLY`: Assigned to a transaction stream that occurs approximately every month.

`ANNUALLY`: Assigned to a transaction stream that occurs approximately every year.

`UNKNOWN`: Assigned to a transaction stream that does not fit any of the pre-defined frequencies.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RecurringTransactionFrequency {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "WEEKLY")]
    Weekly,
    #[serde(rename = "BIWEEKLY")]
    Biweekly,
    #[serde(rename = "SEMI_MONTHLY")]
    SemiMonthly,
    #[serde(rename = "MONTHLY")]
    Monthly,
    #[serde(rename = "ANNUALLY")]
    Annually,
}
