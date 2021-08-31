use super::{AstronomyCurrent, AstronomyDay, AstronomyObjectType};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Information about an astronomical object.
pub struct AstronomyObjectDetails {
    /// Object name.
    pub name: AstronomyObjectType,

    /// Lists and wraps all requested days where events are happening.
    pub days: Option<Vec<AstronomyDay>>,

    /// The current data for the object. Only if requested.
    pub current: Option<AstronomyCurrent>,

    /// The specific data for the object at isotime/utctime.
    pub results: Option<Vec<AstronomyCurrent>>,
}
