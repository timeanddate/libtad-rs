use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Time zone information.
pub struct TimeZone {
    /// The time zone offset (from UTC) in string representation.
    ///
    /// Example: +11:00
    pub offset: String,

    /// Abbreviated time zone name.
    ///
    /// Example: LHDT
    pub zoneabb: String,

    /// Full time zone name.
    ///
    /// Example: Lord Howe Daylight Time
    pub zonename: Option<String>,

    /// Basic time zone offset (without DST) in seconds.
    pub zoneoffset: i32,

    /// DST component of time zone offset in seconds.
    ///
    /// Example: 1800
    pub zonedst: i32,

    /// Total offset from UTC in seconds.
    ///
    /// Example: 39600
    pub zonetotaloffset: i32,
}
