use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::processor_liabilities_get`].

On request success, this will return a [`ProcessorLiabilitiesGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorLiabilitiesGetRequest {
    pub processor_token: String,
}
impl FluentRequest<'_, ProcessorLiabilitiesGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProcessorLiabilitiesGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::ProcessorLiabilitiesGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/liabilities/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "processor_token" : self.params.processor_token }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve Liabilities data

The `/processor/liabilities/get` endpoint returns various details about a loan or credit account. Liabilities data is available primarily for US financial institutions, with some limited coverage of Canadian institutions. Currently supported account types are account type `credit` with account subtype `credit card` or `paypal`, and account type `loan` with account subtype `student` or `mortgage`.

The types of information returned by Liabilities can include balances and due dates, loan terms, and account details such as original loan amount and guarantor. Data is refreshed approximately once per day; the latest data can be retrieved by calling `/processor/liabilities/get`.

Note: This request may take some time to complete if `liabilities` was not specified as an initial product when creating the processor token. This is because Plaid must communicate directly with the institution to retrieve the additional data.

See endpoint docs at <https://plaid.com/docs/api/processor-partners/#processorliabilitiesget>.*/
    pub fn processor_liabilities_get(
        &self,
        processor_token: &str,
    ) -> FluentRequest<'_, ProcessorLiabilitiesGetRequest> {
        FluentRequest {
            client: self,
            params: ProcessorLiabilitiesGetRequest {
                processor_token: processor_token.to_owned(),
            },
        }
    }
}
