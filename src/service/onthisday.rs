use super::Service;
use crate::{ApiError, Error, ServiceClient};
use maybe_async::maybe_async;

mod request;
mod response;

/// On This Day API request.
pub use request::OnThisDayRequest;

/// On This Day API response.
pub use response::OnThisDayResponse;

struct OnThisDayService;

impl Service for OnThisDayService {
    const PATH: &'static str = "onthisday";

    type Request = OnThisDayRequest;
    type Response = OnThisDayResponse;
}

impl ServiceClient {
    #[maybe_async]
    /// The *On This Day* service can be used to retrieve a list of events, births and deaths for a
    /// given day.
    pub async fn get_events_on_this_day(
        &self,
        request: &OnThisDayRequest,
    ) -> Result<Result<OnThisDayResponse, ApiError>, Error> {
        self.call::<OnThisDayService>(request).await
    }
}
