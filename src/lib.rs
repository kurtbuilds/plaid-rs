// libninja: static
use httpclient::Client;
use std::borrow::Cow;
use std::env::var;
use std::sync::OnceLock;
pub mod model;
pub mod request;
pub fn default_http_client() -> Client {
    Client::new().base_url(
        std::env::var("PLAID_ENV")
            .expect("Missing environment variable PLAID_ENV")
            .as_str(),
    )
}
static SHARED_HTTPCLIENT: OnceLock<Client> = OnceLock::new();
/// Use this method if you want to add custom middleware to the httpclient.
/// It must be called before any requests are made, otherwise it will have no effect.
/// Example usage:
///
/// ```
/// init_http_client(default_http_client()
///     .with_middleware(..)
/// );
/// ```
pub fn init_http_client(init: Client) {
    let _ = SHARED_HTTPCLIENT.set(init);
}
fn shared_http_client() -> Cow<'static, Client> {
    Cow::Borrowed(SHARED_HTTPCLIENT.get_or_init(default_http_client))
}
#[derive(Clone)]
pub struct FluentRequest<'a, T> {
    pub(crate) client: &'a PlaidClient,
    pub params: T,
}
pub struct PlaidClient {
    client: Cow<'static, Client>,
    authentication: PlaidAuth,
}
impl PlaidClient {
    pub fn from_env() -> Self {
        Self {
            client: shared_http_client(),
            authentication: PlaidAuth::from_env(),
        }
    }
    pub fn with_auth(authentication: PlaidAuth) -> Self {
        Self {
            client: shared_http_client(),
            authentication,
        }
    }
    pub fn new(client: Client, authentication: PlaidAuth) -> Self {
        Self {
            client: Cow::Owned(client),
            authentication,
        }
    }
}
impl PlaidClient {
    pub(crate) fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        match &self.authentication {
            PlaidAuth::ClientId {
                client_id,
                secret,
                version,
            } => {
                r = r.header("PLAID-CLIENT-ID", client_id);
                r = r.header("PLAID-SECRET", secret);
                r = r.header("Plaid-Version", version);
            }
        }
        r
    }
}
pub enum PlaidAuth {
    ClientId {
        client_id: String,
        secret: String,
        version: String,
    },
}

impl PlaidAuth {
    pub fn from_env() -> Self {
        Self::ClientId {
            client_id: var("PLAID_CLIENT_ID").expect("env var PLAID_CLIENT_ID is not set"),
            secret: var("PLAID_SECRET").expect("env var PLAID_SECRET is not set"),
            version: var("PLAID_VERSION").expect("env var PLAID_VERSION is not set"),
        }
    }
}
