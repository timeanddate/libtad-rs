use crate::models::{date_calculator::Period, places::Geo};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// BusinessDate API response.
pub struct BusinessDateResponse {
    /// Geographical information about the location.
    pub geo: Geo,

    /// The calculated result for the requested periods.
    pub periods: Vec<Period>,
}

#[derive(Debug, Deserialize)]
/// BusinessDuration API response.
pub struct BusinessDurationResponse {
    /// Geographical information about the location.
    pub geo: Geo,

    /// The calculated result for the requested period.
    pub period: Period,
}
