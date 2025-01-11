use serde::{Serialize, Deserialize};
///An optional object to be used with the request. If specified, `options` must not be `null`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsSyncRequestOptions {
    /**This field only applies to calls for Items where the Transactions product has not already been initialized (i.e., by specifying `transactions` in the `products`, `required_if_supported_products`, or `optional_products` array when calling `/link/token/create` or by making a previous call to `/transactions/sync` or `/transactions/get`). In those cases, the field controls the maximum number of days of transaction history that Plaid will request from the financial institution. The more transaction history is requested, the longer the historical update poll will take. If no value is specified, 90 days of history will be requested by default.

If you are initializing your Items with transactions during the `/link/token/create` call (e.g. by including `transactions` in the `/link/token/create` `products` array), you must use the [`transactions.days_requested`](https://plaid.com/docs/api/link/#link-token-create-request-transactions-days-requested) field in the `/link/token/create` request instead of in the `/transactions/sync` request.

If the Item has already been initialized with the Transactions product, this field will have no effect. The maximum amount of transaction history to request on an Item cannot be updated if Transactions has already been added to the Item. To request older transaction history on an Item where Transactions has already been added, you must delete the Item via `/item/remove` and send the user through Link to create a new Item.

Customers using [Recurring Transactions](https://plaid.com/docs/api/products/transactions/#transactionsrecurringget) should request at least 180 days of history for optimal results.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_requested: Option<i64>,
    ///Counterparties and extra merchant fields are now returned by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_logo_and_counterparty_beta: Option<bool>,
    ///Include the raw unparsed transaction description from the financial institution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_original_description: Option<bool>,
    ///Personal finance categories are now returned by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_personal_finance_category: Option<bool>,
}
impl std::fmt::Display for TransactionsSyncRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
