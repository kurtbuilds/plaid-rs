use serde::{Serialize, Deserialize};
///An optional object for `/credit/bank_statements/uploads/get` request options.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankStatementsUploadsGetRequestOptions {
    ///An array of `item_id`s whose bank statements information is returned. Each `item_id` should uniquely identify a bank statements uploaded item. If this field is not provided, all `item_id`s associated with the `user_token` will returned in the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_ids: Option<Vec<String>>,
}
impl std::fmt::Display for CreditBankStatementsUploadsGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
