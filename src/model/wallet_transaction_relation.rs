use serde::{Serialize, Deserialize};
/**Transactions are related when they have a logical connection.

For example, a `PAYOUT` transaction can be returned by the sender, creating a `RETURN` transaction. Each `PAYOUT` transaction can have at most one corresponding `RETURN` transaction in case of reversal.

These relationships are bi-directional, meaning that both entities have references to each other. For instance, when a transaction of type RETURN occurs, it is linked to the original transaction being returned. Likewise, the original transaction has a reference back to the RETURN transaction that represents the return.

This field is only populated for transactions of type `RETURN`, `FUNDS_SWEEP`, `REFUND` and `PAYOUT`.

The relationship between a `PIS_PAY_IN` payment and its corresponding `REFUND` transactions is only available through `refund_ids` property in the payment object. See[`/payment_initiation/payment/get`](https://plaid.com/docs/api/products/payment-initiation/#payment_initiation-payment-get-response-refund-ids).*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletTransactionRelation {
    ///The ID of the related transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///The type of the transaction.
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for WalletTransactionRelation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
