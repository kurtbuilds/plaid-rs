use serde::{Serialize, Deserialize};
///The file type for risk signal analysis
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RiskSignalFileType {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "IMAGE_PDF")]
    ImagePdf,
    #[serde(rename = "SCAN_OCR")]
    ScanOcr,
    #[serde(rename = "TRUE_PDF")]
    TruePdf,
    #[serde(rename = "IMAGE")]
    Image,
    #[serde(rename = "MIXED_PAGE_PDF")]
    MixedPagePdf,
    #[serde(rename = "EMPTY_PDF")]
    EmptyPdf,
    #[serde(rename = "FLATTENED_PDF")]
    FlattenedPdf,
}
