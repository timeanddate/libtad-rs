use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
/// All valid event types.
pub enum EventType {
    /// A historical event.
    Events,

    /// Birth of a historical person.
    Births,

    /// Death of a historical person.
    Deaths,
}
