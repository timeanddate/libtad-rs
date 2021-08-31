use super::AstronomyEventType;
use serde::Deserialize;

#[derive(Deserialize)]
/// Information about a sunrise/sunset event for a specific day.
pub struct AstronomyEvent {
    /// Indicates the type of the event.
    pub r#type: AstronomyEventType,

    /// Hour at which the event is happening (local time).
    pub hour: i32,

    /// Minute at which the event is happening (local time).
    pub minute: i32,
}
