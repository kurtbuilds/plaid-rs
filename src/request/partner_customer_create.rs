use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{
    PartnerEndCustomerAddress, PartnerEndCustomerAssetsUnderManagement,
    PartnerEndCustomerBillingContact, PartnerEndCustomerCustomerSupportInfo, Products,
    PartnerEndCustomerTechnicalContact,
};
/**You should use this struct via [`PlaidClient::partner_customer_create`].

On request success, this will return a [`PartnerCustomerCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerCustomerCreateRequest {
    pub address: PartnerEndCustomerAddress,
    pub application_name: String,
    pub assets_under_management: Option<PartnerEndCustomerAssetsUnderManagement>,
    pub billing_contact: Option<PartnerEndCustomerBillingContact>,
    pub client_id: Option<String>,
    pub company_name: String,
    pub create_link_customization: Option<bool>,
    pub customer_support_info: Option<PartnerEndCustomerCustomerSupportInfo>,
    pub is_bank_addendum_completed: bool,
    pub is_diligence_attested: bool,
    pub legal_entity_name: String,
    pub logo: Option<String>,
    pub products: Option<Vec<Products>>,
    pub redirect_uris: Option<Vec<String>>,
    pub registration_number: Option<String>,
    pub secret: Option<String>,
    pub technical_contact: Option<PartnerEndCustomerTechnicalContact>,
    pub website: String,
}
pub struct PartnerCustomerCreateRequired<'a> {
    pub address: PartnerEndCustomerAddress,
    pub application_name: &'a str,
    pub company_name: &'a str,
    pub is_bank_addendum_completed: bool,
    pub is_diligence_attested: bool,
    pub legal_entity_name: &'a str,
    pub website: &'a str,
}
impl FluentRequest<'_, PartnerCustomerCreateRequest> {
    ///Set the value of the assets_under_management field.
    pub fn assets_under_management(
        mut self,
        assets_under_management: PartnerEndCustomerAssetsUnderManagement,
    ) -> Self {
        self.params.assets_under_management = Some(assets_under_management);
        self
    }
    ///Set the value of the billing_contact field.
    pub fn billing_contact(
        mut self,
        billing_contact: PartnerEndCustomerBillingContact,
    ) -> Self {
        self.params.billing_contact = Some(billing_contact);
        self
    }
    ///Set the value of the client_id field.
    pub fn client_id(mut self, client_id: &str) -> Self {
        self.params.client_id = Some(client_id.to_owned());
        self
    }
    ///Set the value of the create_link_customization field.
    pub fn create_link_customization(mut self, create_link_customization: bool) -> Self {
        self.params.create_link_customization = Some(create_link_customization);
        self
    }
    ///Set the value of the customer_support_info field.
    pub fn customer_support_info(
        mut self,
        customer_support_info: PartnerEndCustomerCustomerSupportInfo,
    ) -> Self {
        self.params.customer_support_info = Some(customer_support_info);
        self
    }
    ///Set the value of the logo field.
    pub fn logo(mut self, logo: &str) -> Self {
        self.params.logo = Some(logo.to_owned());
        self
    }
    ///Set the value of the products field.
    pub fn products(mut self, products: Vec<Products>) -> Self {
        self.params.products = Some(products);
        self
    }
    ///Set the value of the redirect_uris field.
    pub fn redirect_uris(
        mut self,
        redirect_uris: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .redirect_uris = Some(
            redirect_uris.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    ///Set the value of the registration_number field.
    pub fn registration_number(mut self, registration_number: &str) -> Self {
        self.params.registration_number = Some(registration_number.to_owned());
        self
    }
    ///Set the value of the secret field.
    pub fn secret(mut self, secret: &str) -> Self {
        self.params.secret = Some(secret.to_owned());
        self
    }
    ///Set the value of the technical_contact field.
    pub fn technical_contact(
        mut self,
        technical_contact: PartnerEndCustomerTechnicalContact,
    ) -> Self {
        self.params.technical_contact = Some(technical_contact);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PartnerCustomerCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::PartnerCustomerCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/partner/customer/create";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "address" : self.params.address }));
            r = r
                .json(
                    serde_json::json!(
                        { "application_name" : self.params.application_name }
                    ),
                );
            if let Some(ref unwrapped) = self.params.assets_under_management {
                r = r.json(serde_json::json!({ "assets_under_management" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.billing_contact {
                r = r.json(serde_json::json!({ "billing_contact" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.client_id {
                r = r.json(serde_json::json!({ "client_id" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "company_name" : self.params.company_name }));
            if let Some(ref unwrapped) = self.params.create_link_customization {
                r = r
                    .json(
                        serde_json::json!({ "create_link_customization" : unwrapped }),
                    );
            }
            if let Some(ref unwrapped) = self.params.customer_support_info {
                r = r.json(serde_json::json!({ "customer_support_info" : unwrapped }));
            }
            r = r
                .json(
                    serde_json::json!(
                        { "is_bank_addendum_completed" : self.params
                        .is_bank_addendum_completed }
                    ),
                );
            r = r
                .json(
                    serde_json::json!(
                        { "is_diligence_attested" : self.params.is_diligence_attested }
                    ),
                );
            r = r
                .json(
                    serde_json::json!(
                        { "legal_entity_name" : self.params.legal_entity_name }
                    ),
                );
            if let Some(ref unwrapped) = self.params.logo {
                r = r.json(serde_json::json!({ "logo" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.products {
                r = r.json(serde_json::json!({ "products" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.redirect_uris {
                r = r.json(serde_json::json!({ "redirect_uris" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.registration_number {
                r = r.json(serde_json::json!({ "registration_number" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.secret {
                r = r.json(serde_json::json!({ "secret" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.technical_contact {
                r = r.json(serde_json::json!({ "technical_contact" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "website" : self.params.website }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Creates a new end customer for a Plaid reseller.

The `/partner/customer/create` endpoint is used by reseller partners to create end customers. To create end customers, it should be called in the Production environment only, even when creating Sandbox API keys. If called in the Sandbox environment, it will return a sample response, but no customer will be created and the API keys will not be valid.

See endpoint docs at <https://plaid.com/docs/api/partner/#partnercustomercreate>.*/
    pub fn partner_customer_create(
        &self,
        args: PartnerCustomerCreateRequired,
    ) -> FluentRequest<'_, PartnerCustomerCreateRequest> {
        FluentRequest {
            client: self,
            params: PartnerCustomerCreateRequest {
                address: args.address,
                application_name: args.application_name.to_owned(),
                assets_under_management: None,
                billing_contact: None,
                client_id: None,
                company_name: args.company_name.to_owned(),
                create_link_customization: None,
                customer_support_info: None,
                is_bank_addendum_completed: args.is_bank_addendum_completed,
                is_diligence_attested: args.is_diligence_attested,
                legal_entity_name: args.legal_entity_name.to_owned(),
                logo: None,
                products: None,
                redirect_uris: None,
                registration_number: None,
                secret: None,
                technical_contact: None,
                website: args.website.to_owned(),
            },
        }
    }
}
