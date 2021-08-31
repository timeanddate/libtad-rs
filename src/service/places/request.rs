use serde::Serialize;

#[derive(Default, Serialize)]
/// Places API request.
///
/// Request is validated when supplied to the client.
///
/// Example:
/// ```
/// use libtad_rs::{
///     ServiceClient,
///     service::places::PlacesRequest,
/// };
///
/// let client = ServiceClient::new("access_key".into(), "secret_key".into());
/// let request = PlacesRequest::new()
///     .with_placeid("158")
///     .set_lang("de")
///     .set_geo(false);
///
/// let response = client.get_places(&request);
/// ```
pub struct PlacesRequest {
    placeid: Option<Vec<String>>,
    query: Option<String>,
    qlimit: Option<u8>,
    lang: Option<String>,
    geo: Option<u8>,
}

impl PlacesRequest {
    /// Start building a new request.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the placeid for the request.
    pub fn with_placeid(mut self, placeid: impl Into<String>) -> Self {
        if let Some(ref mut placeids) = self.placeid {
            placeids.push(placeid.into());
        } else {
            self.placeid.insert(vec![placeid.into()]);
        }

        self
    }

    /// Set the query for the request.
    pub fn set_query(mut self, query: impl Into<String>) -> Self {
        self.query.insert(query.into());

        self
    }

    /// Set the maximum number of query results to be returned.
    pub fn set_qlimit(mut self, qlimit: u8) -> Self {
        self.qlimit.insert(qlimit);

        self
    }

    /// Set the request language for the request.
    pub fn set_lang(mut self, lang: impl Into<String>) -> Self {
        self.lang.insert(lang.into());

        self
    }

    /// Toggle whether to return longitude and latitude for the geo object.
    pub fn set_geo(mut self, enable: bool) -> Self {
        self.geo.insert(enable.into());

        self
    }
}
