use libtad_models::onthisday::EventType;
use serde::Serialize;

#[derive(Default, Serialize)]
/// On This Day API request.
///
/// Request is validated when supplied to the client.
///
/// Example:
/// ```
/// use libtad_rs::{
///     ServiceClient,
///     service::onthisday::OnThisDayRequest,
///     models::onthisday::EventType,
/// };
///
/// let client = ServiceClient::new("access_key".into(), "secret_key".into());
/// let request = OnThisDayRequest::new()
///     .set_month(3)
///     .set_day(24)
///     .with_type(EventType::Deaths)
///     .with_type(EventType::Events);
///
/// let response = client.get_events_on_this_day(&request);
/// ```
pub struct OnThisDayRequest {
    month: Option<u8>,
    day: Option<u8>,
    lang: Option<Vec<String>>,
    types: Option<Vec<EventType>>,
}

impl OnThisDayRequest {
    /// Start building a new request.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set month for the request.
    pub fn set_month(mut self, month: u8) -> Self {
        self.month.insert(month);

        self
    }

    /// Set day for the request.
    pub fn set_day(mut self, day: u8) -> Self {
        self.day.insert(day);

        self
    }

    /// Add a request language to the request.
    pub fn with_lang(mut self, lang: impl Into<String>) -> Self {
        if let Some(ref mut langs) = self.lang {
            langs.push(lang.into());
        } else {
            self.lang.insert(vec![lang.into()]);
        }

        self
    }

    /// Add an event type to the request.
    pub fn with_type(mut self, event_type: EventType) -> Self {
        if let Some(ref mut types) = self.types {
            types.push(event_type);
        } else {
            self.types.insert(vec![event_type]);
        }

        self
    }
}
