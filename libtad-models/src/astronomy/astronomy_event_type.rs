use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
/// Indicates the type of an astronomical event.
pub enum AstronomyEventType {
    /// Rise.
    Rise,

    /// Set.
    Set,
}
