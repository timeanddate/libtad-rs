use crate::models::astronomy::AstronomyLocation;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Astronomy API response.
pub struct AstronomyResponse {
    /// Information for the requested locations.
    pub locations: Vec<AstronomyLocation>,
}
