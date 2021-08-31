use super::AstronomyObjectDetails;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Requested astronomical information.
pub struct AstronomyDetails {
    /// Astronomical information for the requested objects.
    pub objects: Vec<AstronomyObjectDetails>,
}
