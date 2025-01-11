use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{
    AchClass, TransferDevice, TransferRecurringNetwork, TransferRecurringSchedule,
    TransferType, TransferUserInRequest,
};
/**You should use this struct via [`PlaidClient::transfer_recurring_create`].

On request success, this will return a [`TransferRecurringCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRecurringCreateRequest {
    pub access_token: String,
    pub account_id: String,
    pub ach_class: Option<AchClass>,
    pub amount: String,
    pub description: String,
    pub device: Option<TransferDevice>,
    pub funding_account_id: Option<String>,
    pub idempotency_key: String,
    pub iso_currency_code: Option<String>,
    pub network: TransferRecurringNetwork,
    pub schedule: TransferRecurringSchedule,
    pub test_clock_id: Option<String>,
    pub type_: TransferType,
    pub user: TransferUserInRequest,
    pub user_present: Option<bool>,
}
pub struct TransferRecurringCreateRequired<'a> {
    pub access_token: &'a str,
    pub account_id: &'a str,
    pub amount: &'a str,
    pub description: &'a str,
    pub idempotency_key: &'a str,
    pub network: TransferRecurringNetwork,
    pub schedule: TransferRecurringSchedule,
    pub type_: TransferType,
    pub user: TransferUserInRequest,
}
impl FluentRequest<'_, TransferRecurringCreateRequest> {
    ///Set the value of the ach_class field.
    pub fn ach_class(mut self, ach_class: AchClass) -> Self {
        self.params.ach_class = Some(ach_class);
        self
    }
    ///Set the value of the device field.
    pub fn device(mut self, device: TransferDevice) -> Self {
        self.params.device = Some(device);
        self
    }
    ///Set the value of the funding_account_id field.
    pub fn funding_account_id(mut self, funding_account_id: &str) -> Self {
        self.params.funding_account_id = Some(funding_account_id.to_owned());
        self
    }
    ///Set the value of the iso_currency_code field.
    pub fn iso_currency_code(mut self, iso_currency_code: &str) -> Self {
        self.params.iso_currency_code = Some(iso_currency_code.to_owned());
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
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferRecurringCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransferRecurringCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/recurring/create";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = r.json(serde_json::json!({ "account_id" : self.params.account_id }));
            if let Some(ref unwrapped) = self.params.ach_class {
                r = r.json(serde_json::json!({ "ach_class" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "amount" : self.params.amount }));
            r = r.json(serde_json::json!({ "description" : self.params.description }));
            if let Some(ref unwrapped) = self.params.device {
                r = r.json(serde_json::json!({ "device" : unwrapped }));
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
            if let Some(ref unwrapped) = self.params.iso_currency_code {
                r = r.json(serde_json::json!({ "iso_currency_code" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "network" : self.params.network }));
            r = r.json(serde_json::json!({ "schedule" : self.params.schedule }));
            if let Some(ref unwrapped) = self.params.test_clock_id {
                r = r.json(serde_json::json!({ "test_clock_id" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "type" : self.params.type_ }));
            r = r.json(serde_json::json!({ "user" : self.params.user }));
            if let Some(ref unwrapped) = self.params.user_present {
                r = r.json(serde_json::json!({ "user_present" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create a recurring transfer

Use the `/transfer/recurring/create` endpoint to initiate a new recurring transfer. This capability is not currently supported for Transfer UI or Platform Payments (beta) customers.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/recurring-transfers/#transferrecurringcreate>.*/
    pub fn transfer_recurring_create(
        &self,
        args: TransferRecurringCreateRequired,
    ) -> FluentRequest<'_, TransferRecurringCreateRequest> {
        FluentRequest {
            client: self,
            params: TransferRecurringCreateRequest {
                access_token: args.access_token.to_owned(),
                account_id: args.account_id.to_owned(),
                ach_class: None,
                amount: args.amount.to_owned(),
                description: args.description.to_owned(),
                device: None,
                funding_account_id: None,
                idempotency_key: args.idempotency_key.to_owned(),
                iso_currency_code: None,
                network: args.network,
                schedule: args.schedule,
                test_clock_id: None,
                type_: args.type_,
                user: args.user,
                user_present: None,
            },
        }
    }
}
