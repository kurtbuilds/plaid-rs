use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transfer_ledger_distribute`].

On request success, this will return a [`TransferLedgerDistributeResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferLedgerDistributeRequest {
    pub amount: String,
    pub description: Option<String>,
    pub from_ledger_id: String,
    pub idempotency_key: String,
    pub to_ledger_id: String,
}
pub struct TransferLedgerDistributeRequired<'a> {
    pub amount: &'a str,
    pub from_ledger_id: &'a str,
    pub idempotency_key: &'a str,
    pub to_ledger_id: &'a str,
}
impl FluentRequest<'_, TransferLedgerDistributeRequest> {
    ///Set the value of the description field.
    pub fn description(mut self, description: &str) -> Self {
        self.params.description = Some(description.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferLedgerDistributeRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransferLedgerDistributeResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/ledger/distribute";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "amount" : self.params.amount }));
            if let Some(ref unwrapped) = self.params.description {
                r = r.json(serde_json::json!({ "description" : unwrapped }));
            }
            r = r
                .json(
                    serde_json::json!({ "from_ledger_id" : self.params.from_ledger_id }),
                );
            r = r
                .json(
                    serde_json::json!(
                        { "idempotency_key" : self.params.idempotency_key }
                    ),
                );
            r = r.json(serde_json::json!({ "to_ledger_id" : self.params.to_ledger_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Move available balance between the ledgers of the platform and one of its originators

Use the `/transfer/ledger/distribute` endpoint to move available balance between the ledgers of the platform and one of its originators.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/ledger/#transferledgerdistribute>.*/
    pub fn transfer_ledger_distribute(
        &self,
        args: TransferLedgerDistributeRequired,
    ) -> FluentRequest<'_, TransferLedgerDistributeRequest> {
        FluentRequest {
            client: self,
            params: TransferLedgerDistributeRequest {
                amount: args.amount.to_owned(),
                description: None,
                from_ledger_id: args.from_ledger_id.to_owned(),
                idempotency_key: args.idempotency_key.to_owned(),
                to_ledger_id: args.to_ledger_id.to_owned(),
            },
        }
    }
}
