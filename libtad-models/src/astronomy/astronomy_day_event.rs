use crate::time::DateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Information about an astronomical event at a specific day.
pub struct AstronomyDayEvent {
    /// Indicates the type of the event.
    pub r#type: String,

    /// Hour at which the event is happening (local time).
    pub hour: i32,

    /// Minute at which the event is happening (local time).
    pub min: i32,

    /// Second at which the event is happening (local time).
    pub sec: i32,

    #[serde(deserialize_with = "DateTime::option_deserialize_from_str", default)]
    /// Local time at which the event is happening in ISO 8601 format.
    /// Only returned if requested by specifying the parameter isotime.
    pub isotime: Option<DateTime>,

    #[serde(deserialize_with = "DateTime::option_deserialize_from_str", default)]
    /// UTC time at which the event is happening in ISO 8601 format.
    /// Only returned if requested by specifying the parameter utctime.
    pub utctime: Option<DateTime>,

    /// Altitude of the center of the queried astronomical object above an ideal horizon.
    /// Only for meridian type events.
    pub altitude: Option<f32>,

    /// Horizontal direction of the astronomical object at set/rise time (referring to true north).
    /// North is 0 degrees, east is 90 degrees, south is 180 degrees and west is 270 degrees.
    /// Only for rise ans set type events.
    pub azimuth: Option<f32>,

    /// Distance of the earth's center to the center of the queried astronomical object in
    /// kilometers. Only for meridian type events.
    pub distance: Option<f32>,

    /// The fractin of the Moon's surface illuminated by the Sun's rays as seen from the selected
    /// location. Only for the moon for meridian type events.
    pub illuminated: Option<f32>,

    /// The counterclockwise angle of the midpoint of the Moon's bright limb as seen from the
    /// selected location. Only for the moon for meridian type events.
    pub posangle: Option<f32>,
}
