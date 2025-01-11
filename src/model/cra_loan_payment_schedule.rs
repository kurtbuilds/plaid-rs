use serde::{Serialize, Deserialize};
/**The frequency of a loan's payment schedule.
`BIWEEKLY` represents one payment every two weeks.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CraLoanPaymentSchedule {
    #[serde(rename = "DAILY")]
    Daily,
    #[serde(rename = "WEEKLY")]
    Weekly,
    #[serde(rename = "BIWEEKLY")]
    Biweekly,
    #[serde(rename = "MONTHLY")]
    Monthly,
    #[serde(rename = "QUARTERLY")]
    Quarterly,
    #[serde(rename = "ANNUALLY")]
    Annually,
    #[serde(rename = "OTHER")]
    Other,
}
