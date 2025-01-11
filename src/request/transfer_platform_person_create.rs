use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{
    TransferPlatformPersonAddress, TransferPlatformPersonIdNumber,
    TransferPlatformPersonName,
};
/**You should use this struct via [`PlaidClient::transfer_platform_person_create`].

On request success, this will return a [`TransferPlatformPersonCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferPlatformPersonCreateRequest {
    pub address: Option<TransferPlatformPersonAddress>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub email_address: Option<String>,
    pub id_number: Option<TransferPlatformPersonIdNumber>,
    pub name: Option<TransferPlatformPersonName>,
    pub originator_client_id: String,
    pub phone_number: Option<String>,
    pub relationship_to_originator: Option<String>,
}
impl FluentRequest<'_, TransferPlatformPersonCreateRequest> {
    ///Set the value of the address field.
    pub fn address(mut self, address: TransferPlatformPersonAddress) -> Self {
        self.params.address = Some(address);
        self
    }
    ///Set the value of the date_of_birth field.
    pub fn date_of_birth(mut self, date_of_birth: chrono::NaiveDate) -> Self {
        self.params.date_of_birth = Some(date_of_birth);
        self
    }
    ///Set the value of the email_address field.
    pub fn email_address(mut self, email_address: &str) -> Self {
        self.params.email_address = Some(email_address.to_owned());
        self
    }
    ///Set the value of the id_number field.
    pub fn id_number(mut self, id_number: TransferPlatformPersonIdNumber) -> Self {
        self.params.id_number = Some(id_number);
        self
    }
    ///Set the value of the name field.
    pub fn name(mut self, name: TransferPlatformPersonName) -> Self {
        self.params.name = Some(name);
        self
    }
    ///Set the value of the phone_number field.
    pub fn phone_number(mut self, phone_number: &str) -> Self {
        self.params.phone_number = Some(phone_number.to_owned());
        self
    }
    ///Set the value of the relationship_to_originator field.
    pub fn relationship_to_originator(
        mut self,
        relationship_to_originator: &str,
    ) -> Self {
        self
            .params
            .relationship_to_originator = Some(relationship_to_originator.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferPlatformPersonCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransferPlatformPersonCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/platform/person/create";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.address {
                r = r.json(serde_json::json!({ "address" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.date_of_birth {
                r = r.json(serde_json::json!({ "date_of_birth" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.email_address {
                r = r.json(serde_json::json!({ "email_address" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.id_number {
                r = r.json(serde_json::json!({ "id_number" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.name {
                r = r.json(serde_json::json!({ "name" : unwrapped }));
            }
            r = r
                .json(
                    serde_json::json!(
                        { "originator_client_id" : self.params.originator_client_id }
                    ),
                );
            if let Some(ref unwrapped) = self.params.phone_number {
                r = r.json(serde_json::json!({ "phone_number" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.relationship_to_originator {
                r = r
                    .json(
                        serde_json::json!({ "relationship_to_originator" : unwrapped }),
                    );
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create a person associated with an originator

Use the `/transfer/platform/person/create` endpoint to create a person record associated with an originator and optionally submit person-specific requirements.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/platform/#transferplatformpersoncreate>.*/
    pub fn transfer_platform_person_create(
        &self,
        originator_client_id: &str,
    ) -> FluentRequest<'_, TransferPlatformPersonCreateRequest> {
        FluentRequest {
            client: self,
            params: TransferPlatformPersonCreateRequest {
                address: None,
                date_of_birth: None,
                email_address: None,
                id_number: None,
                name: None,
                originator_client_id: originator_client_id.to_owned(),
                phone_number: None,
                relationship_to_originator: None,
            },
        }
    }
}
