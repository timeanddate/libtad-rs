use libtad_models::tides::Station;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Tides API response.
pub struct TidesResponse {
    /// List of returned stations.
    pub stations: Vec<Station>,
}
