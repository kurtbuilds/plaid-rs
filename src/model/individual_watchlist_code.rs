use serde::{Serialize, Deserialize};
/**Shorthand identifier for a specific screening list for individuals.
 `AU_CON`: Australia Department of Foreign Affairs and Trade Consolidated List
 `CA_CON`: Government of Canada Consolidated List of Sanctions
 `EU_CON`: European External Action Service Consolidated List
 `IZ_CIA`: CIA List of Chiefs of State and Cabinet Members
 `IZ_IPL`: Interpol Red Notices for Wanted Persons List
 `IZ_PEP`: Politically Exposed Persons List
 `IZ_UNC`: United Nations Consolidated Sanctions
 `IZ_WBK`: World Bank Listing of Ineligible Firms and Individuals
 `UK_HMC`: UK HM Treasury Consolidated List
 `US_DPL`: Bureau of Industry and Security Denied Persons List
 `US_DTC`: US Department of State AECA Debarred
 `US_FBI`: US Department of Justice FBI Wanted List
 `US_FSE`: US OFAC Foreign Sanctions Evaders
 `US_ISN`: US Department of State Nonproliferation Sanctions
 `US_PLC`: US OFAC Palestinian Legislative Council
 `US_SDN`: US OFAC Specially Designated Nationals List
 `US_SSI`: US OFAC Sectoral Sanctions Identifications
 `SG_SOF`: Government of Singapore Terrorists and Terrorist Entities
 `TR_TWL`: Government of Turkey Terrorist Wanted List
 `TR_DFD`: Government of Turkey Domestic Freezing Decisions
 `TR_FOR`: Government of Turkey Foreign Freezing Requests
 `TR_WMD`: Government of Turkey Weapons of Mass Destruction
 `TR_CMB`: Government of Turkey Capital Markets Board*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IndividualWatchlistCode {
    #[serde(rename = "AU_CON")]
    AuCon,
    #[serde(rename = "CA_CON")]
    CaCon,
    #[serde(rename = "EU_CON")]
    EuCon,
    #[serde(rename = "IZ_CIA")]
    IzCia,
    #[serde(rename = "IZ_IPL")]
    IzIpl,
    #[serde(rename = "IZ_PEP")]
    IzPep,
    #[serde(rename = "IZ_UNC")]
    IzUnc,
    #[serde(rename = "IZ_WBK")]
    IzWbk,
    #[serde(rename = "UK_HMC")]
    UkHmc,
    #[serde(rename = "US_DPL")]
    UsDpl,
    #[serde(rename = "US_DTC")]
    UsDtc,
    #[serde(rename = "US_FBI")]
    UsFbi,
    #[serde(rename = "US_FSE")]
    UsFse,
    #[serde(rename = "US_ISN")]
    UsIsn,
    #[serde(rename = "US_MBS")]
    UsMbs,
    #[serde(rename = "US_PLC")]
    UsPlc,
    #[serde(rename = "US_SDN")]
    UsSdn,
    #[serde(rename = "US_SSI")]
    UsSsi,
    #[serde(rename = "SG_SOF")]
    SgSof,
    #[serde(rename = "TR_TWL")]
    TrTwl,
    #[serde(rename = "TR_DFD")]
    TrDfd,
    #[serde(rename = "TR_FOR")]
    TrFor,
    #[serde(rename = "TR_WMD")]
    TrWmd,
    #[serde(rename = "TR_CMB")]
    TrCmb,
}
