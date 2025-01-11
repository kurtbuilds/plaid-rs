use serde::{Serialize, Deserialize};
///Form 1099 Type
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Form1099Type {
    #[serde(rename = "FORM_1099_TYPE_UNKNOWN")]
    Form1099TypeUnknown,
    #[serde(rename = "FORM_1099_TYPE_MISC")]
    Form1099TypeMisc,
    #[serde(rename = "FORM_1099_TYPE_K")]
    Form1099TypeK,
}
