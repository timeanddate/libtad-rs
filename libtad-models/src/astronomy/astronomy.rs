use super::AstronomyObject;
use serde::Deserialize;

#[derive(Deserialize)]
/// Astronomical information for requested objects.
pub struct Astronomy {
    /// List of requested objects.
    pub objects: Vec<AstronomyObject>,
}
