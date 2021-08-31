use super::Country;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Geographical information about a location.
pub struct Geo {
    /// The name of the location.
    pub name: String,

    /// The state of the location within the country (only if applicable).
    pub state: Option<String>,

    /// Country of the location.
    pub country: Country,

    /// Geographical latitude of the location.
    pub latitude: Option<f32>,

    /// Geographical longitude of the location.
    pub longitude: Option<f32>,
}
