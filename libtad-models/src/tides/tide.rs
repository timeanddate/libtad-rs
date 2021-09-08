use super::TidalPhase;
use crate::time::Time;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Information about the tide at a specific point in time.
pub struct Tide {
    /// Date/time of the specific tidal data point.
    pub time: Time,

    /// The elevation of tidal water above or below mean sea level.
    pub amplitude: f32,

    /// The current tidal phase.
    pub phase: TidalPhase,
}
