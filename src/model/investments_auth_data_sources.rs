use serde::{Serialize, Deserialize};
use super::DataSources;
///Object with metadata pertaining to the source of data for the account numbers, owners, and holdings that are returned.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentsAuthDataSources {
    /**A description of the source of data for a given product/data type.

`INSTITUTION`: The institution supports this product, and the data was provided by the institution.
`INSTITUTION_MASK`: The user manually provided the full account number, which was matched to the account mask provided by the institution. Only applicable to the `numbers` data type.
`USER`: The institution does not support this product, and the data was manually provided by the user.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holdings: Option<DataSources>,
    /**A description of the source of data for a given product/data type.

`INSTITUTION`: The institution supports this product, and the data was provided by the institution.
`INSTITUTION_MASK`: The user manually provided the full account number, which was matched to the account mask provided by the institution. Only applicable to the `numbers` data type.
`USER`: The institution does not support this product, and the data was manually provided by the user.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numbers: Option<DataSources>,
    /**A description of the source of data for a given product/data type.

`INSTITUTION`: The institution supports this product, and the data was provided by the institution.
`INSTITUTION_MASK`: The user manually provided the full account number, which was matched to the account mask provided by the institution. Only applicable to the `numbers` data type.
`USER`: The institution does not support this product, and the data was manually provided by the user.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owners: Option<DataSources>,
}
impl std::fmt::Display for InvestmentsAuthDataSources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
