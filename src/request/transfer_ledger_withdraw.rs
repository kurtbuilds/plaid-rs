use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::TransferNetwork;
/**You should use this struct via [`PlaidClient::transfer_ledger_withdraw`].

On request success, this will return a [`TransferLedgerWithdrawResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferLedgerWithdrawRequest {
    pub amount: String,
    pub description: Option<String>,
    pub funding_account_id: Option<String>,
    pub idempotency_key: String,
    pub ledger_id: Option<String>,
    pub network: TransferNetwork,
    pub originator_client_id: Option<String>,
}
impl FluentRequest<'_, TransferLedgerWithdrawRequest> {
    ///Set the value of the description field.
    pub fn description(mut self, description: &str) -> Self {
        self.params.description = Some(description.to_owned());
        self
    }
    ///Set the value of the funding_account_id field.
    pub fn funding_account_id(mut self, funding_account_id: &str) -> Self {
        self.params.funding_account_id = Some(funding_account_id.to_owned());
        self
    }
    ///Set the value of the ledger_id field.
    pub fn ledger_id(mut self, ledger_id: &str) -> Self {
        self.params.ledger_id = Some(ledger_id.to_owned());
        self
    }
    ///Set the value of the originator_client_id field.
    pub fn originator_client_id(mut self, originator_client_id: &str) -> Self {
        self.params.originator_client_id = Some(originator_client_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferLedgerWithdrawRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransferLedgerWithdrawResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/ledger/withdraw";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "amount" : self.params.amount }));
            if let Some(ref unwrapped) = self.params.description {
                r = r.json(serde_json::json!({ "description" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.funding_account_id {
                r = r.json(serde_json::json!({ "funding_account_id" : unwrapped }));
            }
            r = r
                .json(
                    serde_json::json!(
                        { "idempotency_key" : self.params.idempotency_key }
                    ),
                );
            if let Some(ref unwrapped) = self.params.ledger_id {
                r = r.json(serde_json::json!({ "ledger_id" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "network" : self.params.network }));
            if let Some(ref unwrapped) = self.params.originator_client_id {
                r = r.json(serde_json::json!({ "originator_client_id" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Withdraw funds from a Plaid Ledger balance

Use the `/transfer/ledger/withdraw` endpoint to withdraw funds from a Plaid Ledger balance.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/ledger/#transferledgerwithdraw>.*/
    pub fn transfer_ledger_withdraw(
        &self,
        amount: &str,
        idempotency_key: &str,
        network: TransferNetwork,
    ) -> FluentRequest<'_, TransferLedgerWithdrawRequest> {
        FluentRequest {
            client: self,
            params: TransferLedgerWithdrawRequest {
                amount: amount.to_owned(),
                description: None,
                funding_account_id: None,
                idempotency_key: idempotency_key.to_owned(),
                ledger_id: None,
                network,
                originator_client_id: None,
            },
        }
    }
}
