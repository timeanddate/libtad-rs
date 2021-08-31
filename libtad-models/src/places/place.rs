use super::Geo;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Information about a place.
pub struct Place {
    /// Numerical ID of the referenced place.
    pub id: i32,

    /// Textual ID of the referenced place.
    pub urlid: String,

    /// Geographical information about the location.
    pub geo: Geo,
}
