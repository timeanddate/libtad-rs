use super::Service;
use crate::{ApiError, Error, ServiceClient};
use maybe_async::maybe_async;

mod request;
mod response;

/// Holidays API request.
pub use request::HolidaysRequest;

/// Holidays API response.
pub use response::HolidaysResponse;

struct HolidaysService;

impl Service for HolidaysService {
    const PATH: &'static str = "holidays";

    type Request = HolidaysRequest;
    type Response = HolidaysResponse;
}

impl ServiceClient {
    #[maybe_async]
    /// The *Holidays* service can be used to retrieve a list of holidays for a country.
    pub async fn get_holidays(
        &self,
        request: &HolidaysRequest,
    ) -> Result<Result<HolidaysResponse, ApiError>, Error> {
        self.call::<HolidaysService>(request).await
    }
}
