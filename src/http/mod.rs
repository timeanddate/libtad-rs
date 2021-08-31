use crate::{service::Service, ApiError, Error};
use maybe_async::maybe_async;
use serde::Deserialize;
use url::Url;

#[maybe_async]
pub trait HTTPBaseClient {
    async fn get<S: Service>(&self, url: Url) -> Result<Result<S::Response, ApiError>, Error>;
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<S: Service> {
    Err(ApiError),
    Ok(S::Response),
}

impl<S: Service> From<ApiResponse<S>> for Result<S::Response, ApiError> {
    fn from(a: ApiResponse<S>) -> Self {
        match a {
            ApiResponse::Ok(r) => Ok(r),
            ApiResponse::Err(e) => Err(e),
        }
    }
}

// Async

#[cfg(feature = "async-client")]
#[cfg(not(all(feature = "async-client", feature = "sync-client")))]
mod r#async;

#[cfg(feature = "async-client")]
#[cfg(not(all(feature = "async-client", feature = "sync-client")))]
pub use self::r#async::{AsyncClient as HTTPClient, HTTPError};

// Sync

#[cfg(feature = "sync-client")]
#[cfg(not(all(feature = "async-client", feature = "sync-client")))]
mod sync;

#[cfg(feature = "sync-client")]
#[cfg(not(all(feature = "async-client", feature = "sync-client")))]
pub use self::sync::{HTTPError, SyncClient as HTTPClient};

#[cfg(all(feature = "async-client", feature = "sync-client"))]
compile_error!(
    "`async-client` and `sync-client` features cannot both be enabled at \
    the same time, if you want to use `sync-client` you need to set \
    `default-features = false`"
);

#[cfg(not(any(feature = "async-client", feature = "sync-client")))]
compile_error!(
    "You have to enable at least one of the available clients with the \
    `async-client` or `sync-client` features."
);
