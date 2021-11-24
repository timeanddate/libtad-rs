use super::{DateTime, TimeZone};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Information about date, time and timezone.
pub struct Time {
    /// ISO representation of date and time, time zone included
    /// (see <https://dev.timeanddate.com/docs/type-iso#isoTime>)
    /// if different from UTC. If time is not applicable, only the date is shown.
    ///
    /// Example: 2011-06-08T09:18:16+02:00
    /// Example: 2011-06-08T07:18:16 (UTC time)
    /// Example: 2011-06-08 (only date)
    pub iso: String,

    /// Date and time, split up into components.
    pub datetime: DateTime,

    /// Time zone information.
    pub timezone: Option<TimeZone>,
}
