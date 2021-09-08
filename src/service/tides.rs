use super::Service;
use crate::{ApiError, Error, ServiceClient};
use maybe_async::maybe_async;

mod request;
mod response;

/// Tides API request.
pub use request::TidesRequest;

/// Tides API response.
pub use response::TidesResponse;

struct TidesService;

impl Service for TidesService {
    const PATH: &'static str = "tides";

    type Request = TidesRequest;
    type Response = TidesResponse;
}

impl ServiceClient {
    #[maybe_async]
    /// The *Tides* service can be used to retrieve predicted tidal data over a given time
    /// interval for a specific place.
    pub async fn get_tidal_data(
        &self,
        request: &TidesRequest,
    ) -> Result<Result<TidesResponse, ApiError>, Error> {
        self.call::<TidesService>(request).await
    }
}
