use serde::{Serialize, Deserialize};
///The object containing prediction interval data.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraPredictionInterval {
    ///The lower bound of the predicted attribute for the given probability.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lower_bound: Option<f64>,
    /**The probability of the actual value of the attribute falling within the upper and lower bound.
This is a percentage represented as a value between 0 and 1.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probability: Option<f64>,
    ///The upper bound of the predicted attribute for the given probability.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upper_bound: Option<f64>,
}
impl std::fmt::Display for CraPredictionInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
