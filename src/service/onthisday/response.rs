use libtad_models::onthisday::{Event, Person};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// On This Day API response.
pub struct OnThisDayResponse {
    #[serde(default)]
    /// List of returned events.
    pub events: Option<Vec<Event>>,

    #[serde(default)]
    /// List of returned births.
    pub births: Option<Vec<Person>>,

    #[serde(default)]
    /// List of returned deaths.
    pub deaths: Option<Vec<Person>>,
}
