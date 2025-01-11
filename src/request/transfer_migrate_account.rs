use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transfer_migrate_account`].

On request success, this will return a [`TransferMigrateAccountResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferMigrateAccountRequest {
    pub account_number: String,
    pub account_type: String,
    pub routing_number: String,
    pub wire_routing_number: Option<String>,
}
impl FluentRequest<'_, TransferMigrateAccountRequest> {
    ///Set the value of the wire_routing_number field.
    pub fn wire_routing_number(mut self, wire_routing_number: &str) -> Self {
        self.params.wire_routing_number = Some(wire_routing_number.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferMigrateAccountRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransferMigrateAccountResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/migrate_account";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!({ "account_number" : self.params.account_number }),
                );
            r = r.json(serde_json::json!({ "account_type" : self.params.account_type }));
            r = r
                .json(
                    serde_json::json!({ "routing_number" : self.params.routing_number }),
                );
            if let Some(ref unwrapped) = self.params.wire_routing_number {
                r = r.json(serde_json::json!({ "wire_routing_number" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Migrate account into Transfers

As an alternative to adding Items via Link, you can also use the `/transfer/migrate_account` endpoint to migrate known account and routing numbers to Plaid Items. This endpoint is also required when adding an Item for use with wire transfers; if you intend to create wire transfers on this account, you must provide `wire_routing_number`. Note that Items created in this way are not compatible with endpoints for other products, such as `/accounts/balance/get`, and can only be used with Transfer endpoints.  If you require access to other endpoints, create the Item through Link instead.  Access to `/transfer/migrate_account` is not enabled by default; to obtain access, contact your Plaid Account Manager.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/account-linking/#transfermigrate_account>.*/
    pub fn transfer_migrate_account(
        &self,
        account_number: &str,
        account_type: &str,
        routing_number: &str,
    ) -> FluentRequest<'_, TransferMigrateAccountRequest> {
        FluentRequest {
            client: self,
            params: TransferMigrateAccountRequest {
                account_number: account_number.to_owned(),
                account_type: account_type.to_owned(),
                routing_number: routing_number.to_owned(),
                wire_routing_number: None,
            },
        }
    }
}
