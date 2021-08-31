use super::{Country, LocationRef};
use serde::Deserialize;

#[derive(Deserialize)]
/// The geographical region. Contains country, a textual description of the region and the name of
/// the biggest place.
pub struct Region {
    /// Country which the region belongs to.
    pub country: Country,

    /// Textual description of a region.
    ///
    /// Example: All locations
    /// Example: most of Newfoundland and Labrador
    /// Example: some regions of Nunavut Territory; small region of Ontario
    pub desc: String,

    /// Name of the biggest city within the region.
    pub biggestplace: String,

    /// A list of all locations referenced by this region.
    pub locations: Option<Vec<LocationRef>>,
}
