use super::Time;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// UTC time stamp of the requested time.
pub struct Utc {
    /// UTC time stamp in ISO8601 format, and (if requested) split into components.
    pub time: Time,
}
