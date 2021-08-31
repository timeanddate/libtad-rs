use super::MoonPhase;
use crate::time::DateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Current information about an astronomical object.
pub struct AstronomyCurrent {
    #[serde(deserialize_with = "DateTime::option_deserialize_from_str", default)]
    /// Local time stamp for the data in ISO 8601 format (including UTC offset).
    /// Only returned if requested by specifying the parameter isotime.
    ///
    /// Example: 2012-04-17T00:57:42+02:00
    pub isotime: Option<DateTime>,

    #[serde(deserialize_with = "DateTime::option_deserialize_from_str", default)]
    /// UTC time stamp for the data in ISO 8601 format.
    /// Only returned if requested by specifying the parameter utctime.
    pub utctime: Option<DateTime>,

    /// Altitude of the center of the queried astronomical object above an ideal horizon.
    pub altitude: f64,

    /// Horizontal direction of the astronomical object at set/rise time (referring to true north).
    /// North is 0 degrees, east is 90 degrees, south is 180 degrees and west is 270 degrees.
    pub azimuth: f64,

    /// Distance of the earth's center to the center of the queried astronomical object in
    /// kilometers.
    pub distance: f64,

    /// The fraction of the Moon's surface illuminated by the Sun's rays as seen from the selected
    /// location. Only available for the moon object.
    pub illuminated: Option<f64>,

    /// The counterclockwise angle of the midpoint of the Moon's bright limb as seen from the
    /// selected location. Only available for the moon object.
    pub posangle: Option<f64>,

    /// The current phase of the moon. Only available for the moon object.
    pub moonphase: Option<MoonPhase>,
}
