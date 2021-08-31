use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
/// All valid astronomy object types.
pub enum AstronomyObjectType {
    /// The sun.
    Sun,

    /// The moon.
    Moon,

    /// Mercury.
    Mercury,

    /// Venus.
    Venus,

    /// Mars.
    Mars,

    /// Jupiter.
    Jupiter,

    /// Saturn.
    Saturn,

    /// Uranus.
    Uranus,

    /// Neptune.
    Neptune,

    /// Pluto.
    Pluto,
}
