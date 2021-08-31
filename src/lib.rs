#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use http::{HTTPBaseClient, HTTPClient};
use maybe_async::maybe_async;
use service::Service;
use url::Url;

mod error;
mod http;

/// Service-related models.
pub mod models {
    pub use libtad_models::*;
}

/// Error returned from the API.
pub use error::ApiError;

/// Internal error.
pub use error::Error;

/// Available Time and Date services.
pub mod service;

/// Client for accessing the Time and Date APIs.
pub struct ServiceClient {
    client: HTTPClient,
    access_key: String,
    secret_key: String,
}

impl ServiceClient {
    const BASE_URL: &'static str = "https://api.xmltime.com";
    const VERSION: &'static str = "3";

    /// Initialize a new client with an access and secret key.
    pub fn new(access_key: String, secret_key: String) -> Self {
        Self {
            client: HTTPClient::new(),
            access_key,
            secret_key,
        }
    }

    #[maybe_async]
    async fn call<S: Service>(
        &self,
        request: &S::Request,
    ) -> Result<Result<S::Response, ApiError>, Error> {
        let mut url = Url::parse(Self::BASE_URL).unwrap();
        let query = S::build_query(request);

        url.set_path(S::PATH);
        url.set_query(query.as_deref());
        url = self.authenticate::<S>(url)?;

        self.client.get::<S>(url).await
    }

    fn authenticate<S: Service>(&self, mut url: Url) -> Result<Url, Error> {
        use hmac::{Hmac, Mac, NewMac};
        use sha1::Sha1;

        let timestamp = chrono::Utc::now().format("%FT%T");

        let mut mac = Hmac::<Sha1>::new_from_slice(self.secret_key.as_bytes())?;

        let message = format!("{}{}{}", self.access_key, S::PATH, timestamp);
        mac.update(message.as_bytes());

        let bytes = mac.finalize().into_bytes();
        let signature = base64::encode(bytes);

        url.query_pairs_mut()
            .append_pair("accesskey", &self.access_key)
            .append_pair("signature", &signature)
            .append_pair("timestamp", &timestamp.to_string())
            .append_pair("version", Self::VERSION);

        Ok(url)
    }
}
