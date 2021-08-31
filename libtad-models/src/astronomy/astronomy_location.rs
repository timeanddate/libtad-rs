use crate::{astronomy::AstronomyDetails, places::Geo};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Information about requested location and astronomical objects.
pub struct AstronomyLocation {
    /// The id of the location.
    pub id: String,

    /// Geographical information about the location.
    pub geo: Geo,

    /// The part of the queried place that this location matches.
    pub matchparam: String,

    /// Requested astronomical information.
    pub astronomy: AstronomyDetails,
}
