use serde::{Serialize, Deserialize};
/**Specifies the purpose of the uploaded file.

`"DUE_DILIGENCE"` - The transfer due diligence document of the originator.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferDocumentPurpose {
    #[serde(rename = "DUE_DILIGENCE")]
    DueDiligence,
}
