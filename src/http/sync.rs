use attohttpc::{header, Method, RequestBuilder, Response};
use url::Url;

use super::{ApiResponse, HTTPBaseClient};
use crate::{service::Service, ApiError, Error};

pub type HTTPError = attohttpc::Error;

pub struct SyncClient;

impl SyncClient {
    const USER_AGENT: &'static str = concat!("sync-libtad-rs-", env!("CARGO_PKG_VERSION"));

    pub fn new() -> Self {
        Self
    }
}

impl HTTPBaseClient for SyncClient {
    fn get<S: Service>(&self, url: Url) -> Result<Result<S::Response, ApiError>, Error> {
        Ok(RequestBuilder::try_new(Method::GET, url)
            .and_then(|r| RequestBuilder::try_header(r, header::USER_AGENT, Self::USER_AGENT))
            .and_then(RequestBuilder::send)
            .and_then(Response::json::<ApiResponse<S>>)?
            .into())
    }
}
