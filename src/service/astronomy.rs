use super::Service;
use crate::{ApiError, Error, ServiceClient};
use maybe_async::maybe_async;

mod request;
mod response;

/// Astronomy API requests.
pub use request::{AstroEventRequest, AstroPositionRequest};

/// Astronomy API responses.
pub use response::AstronomyResponse;

struct AstroEventService;
struct AstroPositionService;

impl Service for AstroEventService {
    const PATH: &'static str = "astronomy";

    type Request = AstroEventRequest;
    type Response = AstronomyResponse;
}

impl Service for AstroPositionService {
    const PATH: &'static str = "astrodata";

    type Request = AstroPositionRequest;
    type Response = AstronomyResponse;
}

impl ServiceClient {
    #[maybe_async]
    /// The *Astro Event* service can be used to retrieve the sunrise, sunset, moonrise, moonset,
    /// solar noon and twilight times for all locations in our database. The service can also
    /// return the azimuth of events, the altitude, and the distance to the sun for the noon event.
    pub async fn get_astro_events(
        &self,
        request: &AstroEventRequest,
    ) -> Result<Result<AstronomyResponse, ApiError>, Error> {
        self.call::<AstroEventService>(request).await
    }

    #[maybe_async]
    /// The *Astro Position* service can be used to retrieve the altitude, azimuth and distance to
    /// the Moon and the Sun for all locations in our database. The service also returns the moon
    /// phase, the fraction of the Moon's illuminated side as well as the midpoint angle of the
    /// Moon's bright limb at any point in time.
    ///
    /// Unlike the Astro Event service, the Astro Position service can be queried on a specific
    /// point in time, down to the second.
    pub async fn get_astro_position(
        &self,
        request: &AstroPositionRequest,
    ) -> Result<Result<AstronomyResponse, ApiError>, Error> {
        self.call::<AstroPositionService>(request).await
    }
}
