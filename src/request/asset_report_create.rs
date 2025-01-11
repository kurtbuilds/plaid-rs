use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::AssetReportCreateRequestOptions;
/**You should use this struct via [`PlaidClient::asset_report_create`].

On request success, this will return a [`AssetReportCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReportCreateRequest {
    pub access_tokens: Option<Vec<String>>,
    pub days_requested: i64,
    pub options: Option<AssetReportCreateRequestOptions>,
}
impl FluentRequest<'_, AssetReportCreateRequest> {
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
    ///Set the value of the options field.
    pub fn options(mut self, options: AssetReportCreateRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AssetReportCreateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::AssetReportCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/asset_report/create";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.access_tokens {
                r = r.json(serde_json::json!({ "access_tokens" : unwrapped }));
            }
            r = r
                .json(
                    serde_json::json!({ "days_requested" : self.params.days_requested }),
                );
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create an Asset Report

The `/asset_report/create` endpoint initiates the process of creating an Asset Report, which can then be retrieved by passing the `asset_report_token` return value to the `/asset_report/get` or `/asset_report/pdf/get` endpoints.

The Asset Report takes some time to be created and is not available immediately after calling `/asset_report/create`. The exact amount of time to create the report will vary depending on how many days of history are requested and will typically range from a few seconds to about one minute. When the Asset Report is ready to be retrieved using `/asset_report/get` or `/asset_report/pdf/get`, Plaid will fire a `PRODUCT_READY` webhook. For full details of the webhook schema, see [Asset Report webhooks](https://plaid.com/docs/api/products/assets/#webhooks).

The `/asset_report/create` endpoint creates an Asset Report at a moment in time. Asset Reports are immutable. To get an updated Asset Report, use the `/asset_report/refresh` endpoint.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#asset_reportcreate>.*/
    pub fn asset_report_create(
        &self,
        days_requested: i64,
    ) -> FluentRequest<'_, AssetReportCreateRequest> {
        FluentRequest {
            client: self,
            params: AssetReportCreateRequest {
                access_tokens: None,
                days_requested,
                options: None,
            },
        }
    }
}
