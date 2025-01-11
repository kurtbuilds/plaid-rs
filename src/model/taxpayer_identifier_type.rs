use serde::{Serialize, Deserialize};
///A value from a MISMO prescribed list that classifies identification numbers used by the Internal Revenue Service (IRS) in the administration of tax laws. A Social Security number (SSN) is issued by the SSA; all other taxpayer identification numbers are issued by the IRS.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaxpayerIdentifierType {
    IndividualTaxpayerIdentificationNumber,
    SocialSecurityNumber,
}
