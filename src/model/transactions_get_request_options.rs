use serde::{Serialize, Deserialize};
///An optional object to be used with the request. If specified, `options` must not be `null`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsGetRequestOptions {
    /**A list of `account_ids` to retrieve for the Item

Note: An error will be returned if a provided `account_id` is not associated with the Item.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    ///The number of transactions to fetch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /**This field only applies to calls for Items where the Transactions product has not already been initialized (i.e. by specifying `transactions` in the `products`, `optional_products`, or `required_if_consented_products` array when calling `/link/token/create` or by making a previous call to `/transactions/sync` or `/transactions/get`). In those cases, the field controls the maximum number of days of transaction history that Plaid will request from the financial institution. The more transaction history is requested, the longer the historical update poll will take. If no value is specified, 90 days of history will be requested by default. If a value under 30 is provided, a minimum of 30 days of history will be requested.

If you are initializing your Items with transactions during the `/link/token/create` call (e.g. by including `transactions` in the `/link/token/create` `products` array), you must use the [`transactions.days_requested`](https://plaid.com/docs/api/link/#link-token-create-request-transactions-days-requested) field in the `/link/token/create` request instead of in the `/transactions/get` request.

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
    ///Personal finance categories are now returned by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_personal_finance_category_beta: Option<bool>,
    ///The number of transactions to skip. The default value is 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}
impl std::fmt::Display for TransactionsGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
