use crate::{places::Country, time::Time, Text};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// A historical event.
pub struct Event {
    /// Identifier for the event.
    pub id: i32,

    /// List of event names.
    pub name: Vec<Text>,

    /// Date of the event.
    pub date: Time,

    /// Location of the event.
    pub location: Option<String>,

    /// Event categories.
    pub categories: Vec<String>,

    /// Related countries.
    pub countries: Vec<Country>,

    /// List of event descriptions.
    pub description: Vec<Text>,
}
