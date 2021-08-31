use maybe_async::async_impl;
use reqwest::Client;
use url::Url;

use super::{ApiResponse, HTTPBaseClient};
use crate::{service::Service, ApiError, Error};

pub type HTTPError = reqwest::Error;

pub struct AsyncClient {
    client: Client,
}

impl AsyncClient {
    const USER_AGENT: &'static str = concat!("async-libtad-rs-", env!("CARGO_PKG_VERSION"));

    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent(Self::USER_AGENT)
            .build()
            .unwrap();

        Self { client }
    }
}

#[async_impl]
impl HTTPBaseClient for AsyncClient {
    async fn get<S: Service>(&self, url: Url) -> Result<Result<S::Response, ApiError>, Error> {
        Ok(self
            .client
            .get(url)
            .send()
            .await?
            .json::<ApiResponse<S>>()
            .await?
            .into())
    }
}
