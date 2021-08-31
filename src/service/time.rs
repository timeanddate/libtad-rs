use super::Service;
use crate::{ApiError, Error, ServiceClient};
use maybe_async::maybe_async;

mod request;
mod response;

/// Time API requests.
pub use request::{ConvertTimeRequest, DSTListRequest, TimeserviceRequest};

/// Time API responses.
pub use response::{ConvertTimeResponse, DSTListResponse, TimeserviceResponse};

struct ConvertTimeService;
struct DSTListService;
struct TimeserviceService;

impl Service for ConvertTimeService {
    const PATH: &'static str = "converttime";

    type Request = ConvertTimeRequest;
    type Response = ConvertTimeResponse;
}

impl Service for DSTListService {
    const PATH: &'static str = "dstlist";

    type Request = DSTListRequest;
    type Response = DSTListResponse;
}

impl Service for TimeserviceService {
    const PATH: &'static str = "timeservice";

    type Request = TimeserviceRequest;
    type Response = TimeserviceResponse;
}

impl ServiceClient {
    #[maybe_async]
    /// The *ConvertTime* service can be used to convert any time from UTC or any of the supported
    /// locations to any other of the supported locations.
    pub async fn convert_time(
        &self,
        request: &ConvertTimeRequest,
    ) -> Result<Result<ConvertTimeResponse, ApiError>, Error> {
        self.call::<ConvertTimeService>(request).await
    }

    #[maybe_async]
    /// The *DSTList* service can be used to obtain data about time zones for all supported countries in our database.
    /// This includes the start and end date of daylight savings time, and UTC offset for the time zones.
    pub async fn get_daylight_savings_time(
        &self,
        request: &DSTListRequest,
    ) -> Result<Result<DSTListResponse, ApiError>, Error> {
        self.call::<DSTListService>(request).await
    }

    #[maybe_async]
    /// The *Timeservice* service can be used to retrieve the current time in one or more places.
    /// Additionally, information about time zones and related changes and the time of sunrise and sunset can be queried.
    pub async fn get_current_time(
        &self,
        request: &TimeserviceRequest,
    ) -> Result<Result<TimeserviceResponse, ApiError>, Error> {
        self.call::<TimeserviceService>(request).await
    }
}
