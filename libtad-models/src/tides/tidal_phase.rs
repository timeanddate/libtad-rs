use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
/// The tidal phase.
pub enum TidalPhase {
    /// High water.
    High,

    /// Low water.
    Low,

    /// Ebb current.
    Ebb,

    /// Flood current.
    Flood,
}
