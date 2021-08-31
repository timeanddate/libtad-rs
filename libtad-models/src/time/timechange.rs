use super::DateTime;
use serde::Deserialize;

#[derive(Deserialize)]
/// Information about a time change.
pub struct TimeChange {
    /// New DST offset in seconds. Value will be empty if there is no DST for this location.
    pub newdst: Option<i32>,

    /// New time zone offset to UTC in seconds if there is a time zone change for this place.
    /// Otherwise the value will be empty. Time zone changes happen only very rarely, so the field
    /// will be empty on most occasions.
    pub newzone: Option<i32>,

    /// New total offset to UTC in seconds.
    pub newoffset: i32,

    /// Time stamp of transition in UTC time, formatted as ISO 8601 time.
    ///
    /// Example: 2011-03-27T01:00:00
    pub utctime: String,

    /// Local time before transition, formatted as ISO 8601 time.
    ///
    /// Example: 2011-03-27T02:00:00
    pub oldlocaltime: String,

    /// Local time after transition, formatted as ISO 8601 time.
    ///
    /// Example: 2011-03-27T03:00:00
    pub newlocaltime: String,

    /// Verbose representation of the time stamps.
    pub verbose: Option<VerboseTimeChange>,
}

#[derive(Deserialize)]
/// Verbose reprsentation of time change time stamps.
pub struct VerboseTimeChange {
    /// Time stamp of transition in UTC time, split up into components.
    pub utctime: DateTime,

    /// Local time before transition, split up into components.
    pub oldlocaltime: DateTime,

    /// Local time after transition, split up into components.
    pub newlocaltime: DateTime,
}
