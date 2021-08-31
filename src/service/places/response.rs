use libtad_models::places::Place;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Places API response.
pub struct PlacesResponse {
    /// List of returned places.
    pub places: Vec<Place>,
}
