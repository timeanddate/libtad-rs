use super::{StationInfo, Tide};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Predicted data for a given station.
pub struct Station {
    /// The source station for the predicted tidal data.
    pub source: StationInfo,

    /// The part of the queried placeid that this location matches.
    pub matchparam: String,

    /// Requested tidal information.
    pub result: Vec<Tide>,
}
