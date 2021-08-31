use super::{AstronomyEvent, AstronomyObjectType};
use serde::Deserialize;

#[derive(Deserialize)]
/// Astronomical information - sunrise and sunset times.
pub struct AstronomyObject {
    /// Object name. Currently, the sun is the only supported astronomical object.
    pub name: AstronomyObjectType,

    /// Lists all sunrise/sunset events during the day.
    pub events: Vec<AstronomyEvent>,

    /// This element is only present if there are no astronomical events.
    /// In this case it will indicate if the sun is up or down the whole day.
    pub special: Option<String>,
}
