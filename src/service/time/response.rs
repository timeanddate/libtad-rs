use crate::models::{
    places::Location,
    time::{DSTEntry, Utc},
};
use serde::Deserialize;

#[derive(Deserialize)]
/// ConvertTime API response.
pub struct ConvertTimeResponse {
    /// UTC time stamp of requested time.
    pub utc: Utc,

    /// List of returned locations.
    pub locations: Vec<Location>,
}

#[derive(Deserialize)]
/// DSTList API response.
pub struct DSTListResponse {
    /// The DST information for each country or region.
    pub dstlist: Vec<DSTEntry>,
}

#[derive(Deserialize)]
/// Timeservice API response.
pub struct TimeserviceResponse {
    /// List of returned locations.
    pub locations: Vec<Location>,
}
