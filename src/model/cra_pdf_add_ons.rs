use serde::{Serialize, Deserialize};
/**A list of add-ons that can be included in the PDF.

`cra_income_insights`: Include Income Insights report in the PDF.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CraPdfAddOns {
    #[serde(rename = "cra_income_insights")]
    CraIncomeInsights,
}
