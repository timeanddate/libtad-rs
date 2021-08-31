use super::Service;
use crate::{ApiError, Error, ServiceClient};
use maybe_async::maybe_async;

mod request;
mod response;

/// Places API request.
pub use request::PlacesRequest;

/// Places API response.
pub use response::PlacesResponse;

struct PlacesService;

impl Service for PlacesService {
    const PATH: &'static str = "places";

    type Request = PlacesRequest;
    type Response = PlacesResponse;
}

impl ServiceClient {
    #[maybe_async]
    /// The *Places* service can be used to retrieve a list of supported places.
    pub async fn get_places(
        &self,
        request: &PlacesRequest,
    ) -> Result<Result<PlacesResponse, ApiError>, Error> {
        self.call::<PlacesService>(request).await
    }
}
