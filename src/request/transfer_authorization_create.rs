use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{
    ACHClass, TransferCreditFundsSource, TransferAuthorizationDevice, TransferNetwork,
    TransferType, TransferAuthorizationUserInRequest, TransferWireDetails,
};
/**You should use this struct via [`PlaidClient::transfer_authorization_create`].

On request success, this will return a [`TransferAuthorizationCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferAuthorizationCreateRequest {
    pub access_token: String,
    pub account_id: String,
    pub ach_class: Option<AchClass>,
    pub amount: String,
    pub beacon_session_id: Option<String>,
    pub credit_funds_source: Option<TransferCreditFundsSource>,
    pub device: Option<TransferAuthorizationDevice>,
    pub funding_account_id: Option<String>,
    pub idempotency_key: Option<String>,
    pub iso_currency_code: Option<String>,
    pub ledger_id: Option<String>,
    pub network: TransferNetwork,
    pub origination_account_id: Option<String>,
    pub originator_client_id: Option<String>,
    pub payment_profile_token: Option<String>,
    pub test_clock_id: Option<String>,
    pub type_: TransferType,
    pub user: TransferAuthorizationUserInRequest,
    pub user_present: Option<bool>,
    pub wire_details: Option<TransferWireDetails>,
    pub with_guarantee: Option<bool>,
}
pub struct TransferAuthorizationCreateRequired<'a> {
    pub access_token: &'a str,
    pub account_id: &'a str,
    pub amount: &'a str,
    pub network: TransferNetwork,
    pub type_: TransferType,
    pub user: TransferAuthorizationUserInRequest,
}
impl FluentRequest<'_, TransferAuthorizationCreateRequest> {
    ///Set the value of the ach_class field.
    pub fn ach_class(mut self, ach_class: AchClass) -> Self {
        self.params.ach_class = Some(ach_class);
        self
    }
    ///Set the value of the beacon_session_id field.
    pub fn beacon_session_id(mut self, beacon_session_id: &str) -> Self {
        self.params.beacon_session_id = Some(beacon_session_id.to_owned());
        self
    }
    ///Set the value of the credit_funds_source field.
    pub fn credit_funds_source(
        mut self,
        credit_funds_source: TransferCreditFundsSource,
    ) -> Self {
        self.params.credit_funds_source = Some(credit_funds_source);
        self
    }
    ///Set the value of the device field.
    pub fn device(mut self, device: TransferAuthorizationDevice) -> Self {
        self.params.device = Some(device);
        self
    }
    ///Set the value of the funding_account_id field.
    pub fn funding_account_id(mut self, funding_account_id: &str) -> Self {
        self.params.funding_account_id = Some(funding_account_id.to_owned());
        self
    }
    ///Set the value of the idempotency_key field.
    pub fn idempotency_key(mut self, idempotency_key: &str) -> Self {
        self.params.idempotency_key = Some(idempotency_key.to_owned());
        self
    }
    ///Set the value of the iso_currency_code field.
    pub fn iso_currency_code(mut self, iso_currency_code: &str) -> Self {
        self.params.iso_currency_code = Some(iso_currency_code.to_owned());
        self
    }
    ///Set the value of the ledger_id field.
    pub fn ledger_id(mut self, ledger_id: &str) -> Self {
        self.params.ledger_id = Some(ledger_id.to_owned());
        self
    }
    ///Set the value of the origination_account_id field.
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.params.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
    ///Set the value of the originator_client_id field.
    pub fn originator_client_id(mut self, originator_client_id: &str) -> Self {
        self.params.originator_client_id = Some(originator_client_id.to_owned());
        self
    }
    ///Set the value of the payment_profile_token field.
    pub fn payment_profile_token(mut self, payment_profile_token: &str) -> Self {
        self.params.payment_profile_token = Some(payment_profile_token.to_owned());
        self
    }
    ///Set the value of the test_clock_id field.
    pub fn test_clock_id(mut self, test_clock_id: &str) -> Self {
        self.params.test_clock_id = Some(test_clock_id.to_owned());
        self
    }
    ///Set the value of the user_present field.
    pub fn user_present(mut self, user_present: bool) -> Self {
        self.params.user_present = Some(user_present);
        self
    }
    ///Set the value of the wire_details field.
    pub fn wire_details(mut self, wire_details: TransferWireDetails) -> Self {
        self.params.wire_details = Some(wire_details);
        self
    }
    ///Set the value of the with_guarantee field.
    pub fn with_guarantee(mut self, with_guarantee: bool) -> Self {
        self.params.with_guarantee = Some(with_guarantee);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferAuthorizationCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransferAuthorizationCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/authorization/create";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = r.json(serde_json::json!({ "account_id" : self.params.account_id }));
            if let Some(ref unwrapped) = self.params.ach_class {
                r = r.json(serde_json::json!({ "ach_class" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "amount" : self.params.amount }));
            if let Some(ref unwrapped) = self.params.beacon_session_id {
                r = r.json(serde_json::json!({ "beacon_session_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.credit_funds_source {
                r = r.json(serde_json::json!({ "credit_funds_source" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.device {
                r = r.json(serde_json::json!({ "device" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.funding_account_id {
                r = r.json(serde_json::json!({ "funding_account_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.idempotency_key {
                r = r.json(serde_json::json!({ "idempotency_key" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.iso_currency_code {
                r = r.json(serde_json::json!({ "iso_currency_code" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.ledger_id {
                r = r.json(serde_json::json!({ "ledger_id" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "network" : self.params.network }));
            if let Some(ref unwrapped) = self.params.origination_account_id {
                r = r.json(serde_json::json!({ "origination_account_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.originator_client_id {
                r = r.json(serde_json::json!({ "originator_client_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.payment_profile_token {
                r = r.json(serde_json::json!({ "payment_profile_token" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.test_clock_id {
                r = r.json(serde_json::json!({ "test_clock_id" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "type" : self.params.type_ }));
            r = r.json(serde_json::json!({ "user" : self.params.user }));
            if let Some(ref unwrapped) = self.params.user_present {
                r = r.json(serde_json::json!({ "user_present" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.wire_details {
                r = r.json(serde_json::json!({ "wire_details" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.with_guarantee {
                r = r.json(serde_json::json!({ "with_guarantee" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create a transfer authorization

Use the `/transfer/authorization/create` endpoint to authorize a transfer. This endpoint must be called prior to calling `/transfer/create`. The transfer authorization will expire if not used after one hour. (You can contact your account manager to change the default authorization lifetime.)

There are four possible outcomes to calling this endpoint:

  - If the `authorization.decision` in the response is `declined`, the proposed transfer has failed the risk check and you cannot proceed with the transfer.

  - If the `authorization.decision` is `user_action_required`, additional user input is needed, usually to fix a broken bank connection, before Plaid can properly assess the risk. You need to launch Link in update mode to complete the required user action. When calling `/link/token/create` to get a new Link token, instead of providing `access_token` in the request, you should set [`transfer.authorization_id`](https://plaid.com/docs/api/link/#link-token-create-request-transfer-authorization-id) as the `authorization.id`. After the Link flow is completed, you may re-attempt the authorization.

  - If the `authorization.decision` is `approved`, and the `authorization.rationale_code` is `null`, the transfer has passed the risk check and you can proceed to call `/transfer/create`.

  - If the `authorization.decision` is `approved` and the `authorization.rationale_code` is non-`null`, the risk check could not be run: you may proceed with the transfer, but should perform your own risk evaluation. For more details, see the response schema.

In Plaid's Sandbox environment the decisions will be returned as follows:

  - To approve a transfer with `null` rationale code, make an authorization request with an `amount` less than the available balance in the account.

  - To approve a transfer with the rationale code `MANUALLY_VERIFIED_ITEM`, create an Item in Link through the [Same Day Micro-deposits flow](https://plaid.com/docs/auth/coverage/testing/#testing-same-day-micro-deposits).

  - To get an authorization decision of `user_action_required`, [reset the login for an Item](https://plaid.com/docs/sandbox/#item_login_required).

  - To decline a transfer with the rationale code `NSF`, the available balance on the account must be less than the authorization `amount`. See [Create Sandbox test data](https://plaid.com/docs/sandbox/user-custom/) for details on how to customize data in Sandbox.

  - To decline a transfer with the rationale code `RISK`, the available balance on the account must be exactly $0. See [Create Sandbox test data](https://plaid.com/docs/sandbox/user-custom/) for details on how to customize data in Sandbox.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/initiating-transfers/#transferauthorizationcreate>.*/
    pub fn transfer_authorization_create(
        &self,
        args: TransferAuthorizationCreateRequired,
    ) -> FluentRequest<'_, TransferAuthorizationCreateRequest> {
        FluentRequest {
            client: self,
            params: TransferAuthorizationCreateRequest {
                access_token: args.access_token.to_owned(),
                account_id: args.account_id.to_owned(),
                ach_class: None,
                amount: args.amount.to_owned(),
                beacon_session_id: None,
                credit_funds_source: None,
                device: None,
                funding_account_id: None,
                idempotency_key: None,
                iso_currency_code: None,
                ledger_id: None,
                network: args.network,
                origination_account_id: None,
                originator_client_id: None,
                payment_profile_token: None,
                test_clock_id: None,
                type_: args.type_,
                user: args.user,
                user_present: None,
                wire_details: None,
                with_guarantee: None,
            },
        }
    }
}
