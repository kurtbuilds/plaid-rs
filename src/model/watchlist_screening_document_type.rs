use serde::{Serialize, Deserialize};
/**The kind of official document represented by this object.

`birth_certificate` - A certificate of birth

`drivers_license` - A license to operate a motor vehicle

`immigration_number` - Immigration or residence documents

`military_id` - Identification issued by a military group

`other` - Any document not covered by other categories

`passport` - An official passport issue by a government

`personal_identification` - Any generic personal identification that is not covered by other categories

`ration_card` - Identification that entitles the holder to rations

`ssn` - United States Social Security Number

`student_id` - Identification issued by an educational institution

`tax_id` - Identification issued for the purpose of collecting taxes

`travel_document` - Visas, entry permits, refugee documents, etc.

`voter_id` - Identification issued for the purpose of voting*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WatchlistScreeningDocumentType {
    #[serde(rename = "birth_certificate")]
    BirthCertificate,
    #[serde(rename = "drivers_license")]
    DriversLicense,
    #[serde(rename = "immigration_number")]
    ImmigrationNumber,
    #[serde(rename = "military_id")]
    MilitaryId,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "passport")]
    Passport,
    #[serde(rename = "personal_identification")]
    PersonalIdentification,
    #[serde(rename = "ration_card")]
    RationCard,
    #[serde(rename = "ssn")]
    Ssn,
    #[serde(rename = "student_id")]
    StudentId,
    #[serde(rename = "tax_id")]
    TaxId,
    #[serde(rename = "travel_document")]
    TravelDocument,
    #[serde(rename = "voter_id")]
    VoterId,
}
