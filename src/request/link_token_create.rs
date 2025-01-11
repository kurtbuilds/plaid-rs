use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{
    LinkTokenAccountFilters, Products, LinkTokenCreateRequestAuth,
    LinkTokenCreateRequestBaseReport, LinkTokenCreateCardSwitch, CountryCode,
    LinkTokenCreateRequestCraOptions, LinkTokenCreateRequestCreditPartnerInsights,
    LinkTokenCreateRequestDepositSwitch, LinkTokenCreateRequestEmployment,
    LinkTokenEUConfig, LinkTokenCreateHostedLink, LinkTokenCreateIdentity,
    LinkTokenCreateRequestIdentityVerification, LinkTokenCreateRequestIncomeVerification,
    LinkTokenCreateInstitutionData, LinkTokenInvestments, LinkTokenInvestmentsAuth,
    LinkTokenCreateRequestPaymentConfiguration, LinkTokenCreateRequestPaymentInitiation,
    LinkTokenCreateRequestStatements, LinkTokenTransactions,
    LinkTokenCreateRequestTransfer, LinkTokenCreateRequestUpdate,
    LinkTokenCreateRequestUser,
};
/**You should use this struct via [`PlaidClient::link_token_create`].

On request success, this will return a [`LinkTokenCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkTokenCreateRequest {
    pub access_token: Option<String>,
    pub access_tokens: Option<Vec<String>>,
    pub account_filters: Option<LinkTokenAccountFilters>,
    pub additional_consented_products: Option<Vec<Products>>,
    pub android_package_name: Option<String>,
    pub auth: Option<LinkTokenCreateRequestAuth>,
    pub base_report: Option<LinkTokenCreateRequestBaseReport>,
    pub card_switch: Option<LinkTokenCreateCardSwitch>,
    pub client_name: String,
    pub consumer_report_permissible_purpose: Option<serde_json::Value>,
    pub country_codes: Vec<CountryCode>,
    pub cra_enabled: Option<bool>,
    pub cra_options: Option<LinkTokenCreateRequestCraOptions>,
    pub credit_partner_insights: Option<LinkTokenCreateRequestCreditPartnerInsights>,
    pub deposit_switch: Option<LinkTokenCreateRequestDepositSwitch>,
    pub employment: Option<LinkTokenCreateRequestEmployment>,
    pub enable_multi_item_link: Option<bool>,
    pub eu_config: Option<LinkTokenEuConfig>,
    pub financekit_supported: Option<bool>,
    pub hosted_link: Option<LinkTokenCreateHostedLink>,
    pub identity: Option<LinkTokenCreateIdentity>,
    pub identity_verification: Option<LinkTokenCreateRequestIdentityVerification>,
    pub income_verification: Option<LinkTokenCreateRequestIncomeVerification>,
    pub institution_data: Option<LinkTokenCreateInstitutionData>,
    pub institution_id: Option<String>,
    pub investments: Option<LinkTokenInvestments>,
    pub investments_auth: Option<LinkTokenInvestmentsAuth>,
    pub language: String,
    pub link_customization_name: Option<String>,
    pub optional_products: Option<Vec<Products>>,
    pub payment_configuration: Option<LinkTokenCreateRequestPaymentConfiguration>,
    pub payment_initiation: Option<LinkTokenCreateRequestPaymentInitiation>,
    pub products: Option<Vec<Products>>,
    pub redirect_uri: Option<String>,
    pub required_if_supported_products: Option<Vec<Products>>,
    pub statements: Option<LinkTokenCreateRequestStatements>,
    pub transactions: Option<LinkTokenTransactions>,
    pub transfer: Option<LinkTokenCreateRequestTransfer>,
    pub update: Option<LinkTokenCreateRequestUpdate>,
    pub user: LinkTokenCreateRequestUser,
    pub user_token: Option<String>,
    pub webhook: Option<String>,
}
pub struct LinkTokenCreateRequired<'a> {
    pub client_name: &'a str,
    pub country_codes: Vec<CountryCode>,
    pub language: &'a str,
    pub user: LinkTokenCreateRequestUser,
}
impl FluentRequest<'_, LinkTokenCreateRequest> {
    ///Set the value of the access_token field.
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.params.access_token = Some(access_token.to_owned());
        self
    }
    ///Set the value of the access_tokens field.
    pub fn access_tokens(
        mut self,
        access_tokens: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .access_tokens = Some(
            access_tokens.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    ///Set the value of the account_filters field.
    pub fn account_filters(mut self, account_filters: LinkTokenAccountFilters) -> Self {
        self.params.account_filters = Some(account_filters);
        self
    }
    ///Set the value of the additional_consented_products field.
    pub fn additional_consented_products(
        mut self,
        additional_consented_products: Vec<Products>,
    ) -> Self {
        self.params.additional_consented_products = Some(additional_consented_products);
        self
    }
    ///Set the value of the android_package_name field.
    pub fn android_package_name(mut self, android_package_name: &str) -> Self {
        self.params.android_package_name = Some(android_package_name.to_owned());
        self
    }
    ///Set the value of the auth field.
    pub fn auth(mut self, auth: LinkTokenCreateRequestAuth) -> Self {
        self.params.auth = Some(auth);
        self
    }
    ///Set the value of the base_report field.
    pub fn base_report(mut self, base_report: LinkTokenCreateRequestBaseReport) -> Self {
        self.params.base_report = Some(base_report);
        self
    }
    ///Set the value of the card_switch field.
    pub fn card_switch(mut self, card_switch: LinkTokenCreateCardSwitch) -> Self {
        self.params.card_switch = Some(card_switch);
        self
    }
    ///Set the value of the consumer_report_permissible_purpose field.
    pub fn consumer_report_permissible_purpose(
        mut self,
        consumer_report_permissible_purpose: serde_json::Value,
    ) -> Self {
        self
            .params
            .consumer_report_permissible_purpose = Some(
            consumer_report_permissible_purpose,
        );
        self
    }
    ///Set the value of the cra_enabled field.
    pub fn cra_enabled(mut self, cra_enabled: bool) -> Self {
        self.params.cra_enabled = Some(cra_enabled);
        self
    }
    ///Set the value of the cra_options field.
    pub fn cra_options(mut self, cra_options: LinkTokenCreateRequestCraOptions) -> Self {
        self.params.cra_options = Some(cra_options);
        self
    }
    ///Set the value of the credit_partner_insights field.
    pub fn credit_partner_insights(
        mut self,
        credit_partner_insights: LinkTokenCreateRequestCreditPartnerInsights,
    ) -> Self {
        self.params.credit_partner_insights = Some(credit_partner_insights);
        self
    }
    ///Set the value of the deposit_switch field.
    pub fn deposit_switch(
        mut self,
        deposit_switch: LinkTokenCreateRequestDepositSwitch,
    ) -> Self {
        self.params.deposit_switch = Some(deposit_switch);
        self
    }
    ///Set the value of the employment field.
    pub fn employment(mut self, employment: LinkTokenCreateRequestEmployment) -> Self {
        self.params.employment = Some(employment);
        self
    }
    ///Set the value of the enable_multi_item_link field.
    pub fn enable_multi_item_link(mut self, enable_multi_item_link: bool) -> Self {
        self.params.enable_multi_item_link = Some(enable_multi_item_link);
        self
    }
    ///Set the value of the eu_config field.
    pub fn eu_config(mut self, eu_config: LinkTokenEuConfig) -> Self {
        self.params.eu_config = Some(eu_config);
        self
    }
    ///Set the value of the financekit_supported field.
    pub fn financekit_supported(mut self, financekit_supported: bool) -> Self {
        self.params.financekit_supported = Some(financekit_supported);
        self
    }
    ///Set the value of the hosted_link field.
    pub fn hosted_link(mut self, hosted_link: LinkTokenCreateHostedLink) -> Self {
        self.params.hosted_link = Some(hosted_link);
        self
    }
    ///Set the value of the identity field.
    pub fn identity(mut self, identity: LinkTokenCreateIdentity) -> Self {
        self.params.identity = Some(identity);
        self
    }
    ///Set the value of the identity_verification field.
    pub fn identity_verification(
        mut self,
        identity_verification: LinkTokenCreateRequestIdentityVerification,
    ) -> Self {
        self.params.identity_verification = Some(identity_verification);
        self
    }
    ///Set the value of the income_verification field.
    pub fn income_verification(
        mut self,
        income_verification: LinkTokenCreateRequestIncomeVerification,
    ) -> Self {
        self.params.income_verification = Some(income_verification);
        self
    }
    ///Set the value of the institution_data field.
    pub fn institution_data(
        mut self,
        institution_data: LinkTokenCreateInstitutionData,
    ) -> Self {
        self.params.institution_data = Some(institution_data);
        self
    }
    ///Set the value of the institution_id field.
    pub fn institution_id(mut self, institution_id: &str) -> Self {
        self.params.institution_id = Some(institution_id.to_owned());
        self
    }
    ///Set the value of the investments field.
    pub fn investments(mut self, investments: LinkTokenInvestments) -> Self {
        self.params.investments = Some(investments);
        self
    }
    ///Set the value of the investments_auth field.
    pub fn investments_auth(
        mut self,
        investments_auth: LinkTokenInvestmentsAuth,
    ) -> Self {
        self.params.investments_auth = Some(investments_auth);
        self
    }
    ///Set the value of the link_customization_name field.
    pub fn link_customization_name(mut self, link_customization_name: &str) -> Self {
        self.params.link_customization_name = Some(link_customization_name.to_owned());
        self
    }
    ///Set the value of the optional_products field.
    pub fn optional_products(mut self, optional_products: Vec<Products>) -> Self {
        self.params.optional_products = Some(optional_products);
        self
    }
    ///Set the value of the payment_configuration field.
    pub fn payment_configuration(
        mut self,
        payment_configuration: LinkTokenCreateRequestPaymentConfiguration,
    ) -> Self {
        self.params.payment_configuration = Some(payment_configuration);
        self
    }
    ///Set the value of the payment_initiation field.
    pub fn payment_initiation(
        mut self,
        payment_initiation: LinkTokenCreateRequestPaymentInitiation,
    ) -> Self {
        self.params.payment_initiation = Some(payment_initiation);
        self
    }
    ///Set the value of the products field.
    pub fn products(mut self, products: Vec<Products>) -> Self {
        self.params.products = Some(products);
        self
    }
    ///Set the value of the redirect_uri field.
    pub fn redirect_uri(mut self, redirect_uri: &str) -> Self {
        self.params.redirect_uri = Some(redirect_uri.to_owned());
        self
    }
    ///Set the value of the required_if_supported_products field.
    pub fn required_if_supported_products(
        mut self,
        required_if_supported_products: Vec<Products>,
    ) -> Self {
        self
            .params
            .required_if_supported_products = Some(required_if_supported_products);
        self
    }
    ///Set the value of the statements field.
    pub fn statements(mut self, statements: LinkTokenCreateRequestStatements) -> Self {
        self.params.statements = Some(statements);
        self
    }
    ///Set the value of the transactions field.
    pub fn transactions(mut self, transactions: LinkTokenTransactions) -> Self {
        self.params.transactions = Some(transactions);
        self
    }
    ///Set the value of the transfer field.
    pub fn transfer(mut self, transfer: LinkTokenCreateRequestTransfer) -> Self {
        self.params.transfer = Some(transfer);
        self
    }
    ///Set the value of the update field.
    pub fn update(mut self, update: LinkTokenCreateRequestUpdate) -> Self {
        self.params.update = Some(update);
        self
    }
    ///Set the value of the user_token field.
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.params.user_token = Some(user_token.to_owned());
        self
    }
    ///Set the value of the webhook field.
    pub fn webhook(mut self, webhook: &str) -> Self {
        self.params.webhook = Some(webhook.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, LinkTokenCreateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::LinkTokenCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/link/token/create";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.access_token {
                r = r.json(serde_json::json!({ "access_token" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.access_tokens {
                r = r.json(serde_json::json!({ "access_tokens" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.account_filters {
                r = r.json(serde_json::json!({ "account_filters" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.additional_consented_products {
                r = r
                    .json(
                        serde_json::json!(
                            { "additional_consented_products" : unwrapped }
                        ),
                    );
            }
            if let Some(ref unwrapped) = self.params.android_package_name {
                r = r.json(serde_json::json!({ "android_package_name" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.auth {
                r = r.json(serde_json::json!({ "auth" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.base_report {
                r = r.json(serde_json::json!({ "base_report" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.card_switch {
                r = r.json(serde_json::json!({ "card_switch" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "client_name" : self.params.client_name }));
            if let Some(ref unwrapped) = self.params.consumer_report_permissible_purpose
            {
                r = r
                    .json(
                        serde_json::json!(
                            { "consumer_report_permissible_purpose" : unwrapped }
                        ),
                    );
            }
            r = r
                .json(
                    serde_json::json!({ "country_codes" : self.params.country_codes }),
                );
            if let Some(ref unwrapped) = self.params.cra_enabled {
                r = r.json(serde_json::json!({ "cra_enabled" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.cra_options {
                r = r.json(serde_json::json!({ "cra_options" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.credit_partner_insights {
                r = r.json(serde_json::json!({ "credit_partner_insights" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.deposit_switch {
                r = r.json(serde_json::json!({ "deposit_switch" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.employment {
                r = r.json(serde_json::json!({ "employment" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.enable_multi_item_link {
                r = r.json(serde_json::json!({ "enable_multi_item_link" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.eu_config {
                r = r.json(serde_json::json!({ "eu_config" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.financekit_supported {
                r = r.json(serde_json::json!({ "financekit_supported" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.hosted_link {
                r = r.json(serde_json::json!({ "hosted_link" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.identity {
                r = r.json(serde_json::json!({ "identity" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.identity_verification {
                r = r.json(serde_json::json!({ "identity_verification" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.income_verification {
                r = r.json(serde_json::json!({ "income_verification" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.institution_data {
                r = r.json(serde_json::json!({ "institution_data" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.institution_id {
                r = r.json(serde_json::json!({ "institution_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.investments {
                r = r.json(serde_json::json!({ "investments" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.investments_auth {
                r = r.json(serde_json::json!({ "investments_auth" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "language" : self.params.language }));
            if let Some(ref unwrapped) = self.params.link_customization_name {
                r = r.json(serde_json::json!({ "link_customization_name" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.optional_products {
                r = r.json(serde_json::json!({ "optional_products" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.payment_configuration {
                r = r.json(serde_json::json!({ "payment_configuration" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.payment_initiation {
                r = r.json(serde_json::json!({ "payment_initiation" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.products {
                r = r.json(serde_json::json!({ "products" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.redirect_uri {
                r = r.json(serde_json::json!({ "redirect_uri" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.required_if_supported_products {
                r = r
                    .json(
                        serde_json::json!(
                            { "required_if_supported_products" : unwrapped }
                        ),
                    );
            }
            if let Some(ref unwrapped) = self.params.statements {
                r = r.json(serde_json::json!({ "statements" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.transactions {
                r = r.json(serde_json::json!({ "transactions" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.transfer {
                r = r.json(serde_json::json!({ "transfer" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.update {
                r = r.json(serde_json::json!({ "update" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "user" : self.params.user }));
            if let Some(ref unwrapped) = self.params.user_token {
                r = r.json(serde_json::json!({ "user_token" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.webhook {
                r = r.json(serde_json::json!({ "webhook" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create Link Token

The `/link/token/create` endpoint creates a `link_token`, which is required as a parameter when initializing Link. Once Link has been initialized, it returns a `public_token`. For most Plaid products, the `public_token` is saved and exchanged for an `access_token` via `/item/public_token/exchange` as part of the main Link flow. For more details, see the [Link flow overview](https://plaid.com/docs/link/#link-flow-overview).

A `link_token` generated by `/link/token/create` is also used to initialize other Link flows, such as the [update mode](https://plaid.com/docs/link/update-mode) flow for tokens with expired credentials, or the Identity Verification flow.

See endpoint docs at <https://plaid.com/docs/api/link/#linktokencreate>.*/
    pub fn link_token_create(
        &self,
        args: LinkTokenCreateRequired,
    ) -> FluentRequest<'_, LinkTokenCreateRequest> {
        FluentRequest {
            client: self,
            params: LinkTokenCreateRequest {
                access_token: None,
                access_tokens: None,
                account_filters: None,
                additional_consented_products: None,
                android_package_name: None,
                auth: None,
                base_report: None,
                card_switch: None,
                client_name: args.client_name.to_owned(),
                consumer_report_permissible_purpose: None,
                country_codes: args.country_codes,
                cra_enabled: None,
                cra_options: None,
                credit_partner_insights: None,
                deposit_switch: None,
                employment: None,
                enable_multi_item_link: None,
                eu_config: None,
                financekit_supported: None,
                hosted_link: None,
                identity: None,
                identity_verification: None,
                income_verification: None,
                institution_data: None,
                institution_id: None,
                investments: None,
                investments_auth: None,
                language: args.language.to_owned(),
                link_customization_name: None,
                optional_products: None,
                payment_configuration: None,
                payment_initiation: None,
                products: None,
                redirect_uri: None,
                required_if_supported_products: None,
                statements: None,
                transactions: None,
                transfer: None,
                update: None,
                user: args.user,
                user_token: None,
                webhook: None,
            },
        }
    }
}
