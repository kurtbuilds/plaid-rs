#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
use plaid::request::PartnerCustomerCreateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let address = PartnerEndCustomerAddress {
        city: Some("your city".to_owned()),
        country_code: Some("your country code".to_owned()),
        postal_code: Some("your postal code".to_owned()),
        region: Some("your region".to_owned()),
        street: Some("your street".to_owned()),
    };
    let application_name = "your application name";
    let company_name = "your company name";
    let is_bank_addendum_completed = true;
    let is_diligence_attested = true;
    let legal_entity_name = "your legal entity name";
    let website = "your website";
    let response = client
        .partner_customer_create(PartnerCustomerCreateRequired {
            address,
            application_name,
            company_name,
            is_bank_addendum_completed,
            is_diligence_attested,
            legal_entity_name,
            website,
        })
        .assets_under_management(PartnerEndCustomerAssetsUnderManagement {
            amount: 1.0,
            iso_currency_code: "your iso currency code".to_owned(),
        })
        .billing_contact(PartnerEndCustomerBillingContact {
            email: Some("your email".to_owned()),
            family_name: Some("your family name".to_owned()),
            given_name: Some("your given name".to_owned()),
        })
        .client_id("your client id")
        .create_link_customization(true)
        .customer_support_info(PartnerEndCustomerCustomerSupportInfo {
            contact_url: Some("your contact url".to_owned()),
            email: Some("your email".to_owned()),
            link_update_url: Some("your link update url".to_owned()),
            phone_number: Some("your phone number".to_owned()),
        })
        .logo("your logo")
        .products(vec![Products::Assets])
        .redirect_uris(&["your redirect uris"])
        .registration_number("your registration number")
        .secret("your secret")
        .technical_contact(PartnerEndCustomerTechnicalContact {
            email: Some("your email".to_owned()),
            family_name: Some("your family name".to_owned()),
            given_name: Some("your given name".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
