use super::Service;
use crate::{ApiError, Error, ServiceClient};
use maybe_async::maybe_async;

mod request;
mod response;

/// Date Calculator API requests.
pub use request::{BusinessDateRequest, BusinessDurationRequest};

/// Date Calculator API responses.
pub use response::{BusinessDateResponse, BusinessDurationResponse};

struct BusinessDateService;
struct BusinessDurationService;

impl Service for BusinessDateService {
    const PATH: &'static str = "businessdate";

    type Request = BusinessDateRequest;
    type Response = BusinessDateResponse;
}

impl Service for BusinessDurationService {
    const PATH: &'static str = "businessduration";

    type Request = BusinessDurationRequest;
    type Response = BusinessDurationResponse;
}

impl ServiceClient {
    #[maybe_async]
    /// The *BusinessDate* service can be used to find a business date from a specified number of days.
    pub async fn calculate_business_date(
        &self,
        request: &BusinessDateRequest,
    ) -> Result<Result<BusinessDateResponse, ApiError>, Error> {
        self.call::<BusinessDateService>(request).await
    }

    #[maybe_async]
    /// The *BusinessDuration* service can be used to calculate the number of business days between a specified start date and end date.
    pub async fn calculate_business_duration(
        &self,
        request: &BusinessDurationRequest,
    ) -> Result<Result<BusinessDurationResponse, ApiError>, Error> {
        self.call::<BusinessDurationService>(request).await
    }
}
