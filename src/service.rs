use serde::{de::DeserializeOwned, Serialize};

/// Astronomy API.
pub mod astronomy;

/// Date Calculator API.
pub mod date_calculator;

/// Holidays API.
pub mod holidays;

/// On This Day API.
pub mod onthisday;

/// Places API.
pub mod places;

/// Tides API.
pub mod tides;

/// Time API.
pub mod time;

/// Trait implemented by all API services.
pub trait Service {
    /// URL path for a service.
    const PATH: &'static str;

    /// Request type for a service.
    type Request: Serialize;

    /// Response type for a service.
    type Response: DeserializeOwned;

    #[doc(hidden)]
    fn build_query(request: &Self::Request) -> Option<String> {
        serde_url_params::to_string(request).ok()
    }
}

#[derive(Default)]
/// Marker to illustrate that a required argument is missing.
pub struct RequiredArgument;

/// Marker to ensure that a required request argument has been provided.
pub struct ProvidedArgument;
