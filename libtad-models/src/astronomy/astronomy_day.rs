use super::{AstronomyDayEvent, MoonPhase};
use crate::time::DateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Information about an astronomical object for a specific day.
pub struct AstronomyDay {
    #[serde(deserialize_with = "DateTime::deserialize_from_str", default)]
    /// Date for the current information.
    pub date: DateTime,

    /// Length of this day (time between sunrise and sunset). If the sun is not up on this day,
    /// 00:00:00 will be reported. If the sun does not set on this day, the value will read
    /// 24:00:00. Attribute only applies for the sun object and if requested.
    pub daylength: Option<String>,

    /// Moon phase for the day. Only if requested.
    pub moonphase: Option<MoonPhase>,

    /// Lists all events during the day.
    pub events: Vec<AstronomyDayEvent>,
}
