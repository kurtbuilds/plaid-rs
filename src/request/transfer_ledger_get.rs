use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transfer_ledger_get`].

On request success, this will return a [`TransferLedgerGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferLedgerGetRequest {
    pub ledger_id: Option<String>,
    pub originator_client_id: Option<String>,
}
impl FluentRequest<'_, TransferLedgerGetRequest> {
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
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferLedgerGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::TransferLedgerGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/ledger/get";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.ledger_id {
                r = r.json(serde_json::json!({ "ledger_id" : unwrapped }));
            }
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
    /**Retrieve Plaid Ledger balance

Use the `/transfer/ledger/get` endpoint to view a balance on the ledger held with Plaid.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/ledger/#transferledgerget>.*/
    pub fn transfer_ledger_get(&self) -> FluentRequest<'_, TransferLedgerGetRequest> {
        FluentRequest {
            client: self,
            params: TransferLedgerGetRequest {
                ledger_id: None,
                originator_client_id: None,
            },
        }
    }
}
