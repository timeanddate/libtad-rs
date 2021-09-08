use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Information about a station.
pub struct StationInfo {
    /// Station name.
    pub name: String,

    /// Latitude coordinate of the station.
    pub latitude: f32,

    /// Longitude coordinate of the station.
    pub longitude: f32,

    /// Station type. Either reference or subordinate station.
    pub r#type: String,

    /// Distance between request place and this station.
    pub distance: f32,
}
