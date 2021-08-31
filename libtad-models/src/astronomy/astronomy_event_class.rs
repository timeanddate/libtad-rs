use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
/// All valid astronomy event classes.
pub enum AstronomyEventClass {
    /// Combination of all known classes.
    All,

    /// The current phase for the place requested. Additional attributes for illumination (moon), azimuth, distance.
    Current,

    /// Day length. Day length is not reported as an event, but as a separate attribute.
    DayLength,

    /// Meridian (Noon, highest point) and Anti-Meridian (lowest point) events.
    Meridian,

    /// Moon phase events. Additionally to the phase events (only occurring on four days per lunar month), an additional attribute for the current moon phase is reported for every day.
    Phase,

    /// Set and rise events. Event times take atmospheric refraction into account.
    SetRise,

    /// Combination of all 3 twilight classes.
    Twilight,

    /// Civil twilight (-6°).
    Twilight6,

    /// Nautical twilight (-12°).
    Twilight12,

    /// Astronomical twilight (-18°).
    Twilight18,
}
