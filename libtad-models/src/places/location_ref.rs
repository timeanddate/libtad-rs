use serde::Deserialize;

#[derive(Deserialize)]
/// Information about a location referenced by a region.
pub struct LocationRef {
    /// The ID of the location.
    pub id: String,

    /// The name of the location.
    pub name: String,

    /// The state of the location within the country (only if applicable).
    pub state: Option<String>,
}
