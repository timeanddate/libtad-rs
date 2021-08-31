use super::Geo;
use crate::{
    astronomy::Astronomy,
    time::{Time, TimeChange},
};
use serde::Deserialize;

#[derive(Deserialize)]
/// Information about a location.
pub struct Location {
    /// The id of the location.
    pub id: String,

    /// The part of the queried placeid that this location matches.
    #[serde(alias = "matches")]
    pub matchparam: String,

    /// Geographical information about the location.
    pub geo: Geo,

    /// Time information. Only present if requested.
    pub time: Option<Time>,

    /// Time changes (daylight savings time). Only present if requested and information is
    /// available.
    pub timechanges: Option<Vec<TimeChange>>,

    /// Astronomical information - sunrise and sunset times. Only for the timeservice and if
    /// requested.
    pub astronomy: Option<Astronomy>,
}
