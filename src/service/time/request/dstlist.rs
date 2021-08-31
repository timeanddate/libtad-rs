use serde::Serialize;

#[derive(Default, Serialize)]
/// DSTList API request.
///
/// Example:
/// ```
/// use libtad_rs::{
///     ServiceClient,
///     service::time::DSTListRequest,
/// };
///
/// let client = ServiceClient::new("access_key".into(), "secret_key".into());
/// let request = DSTListRequest::new()
///     .set_year(2021)
///     .set_onlydst(true);
///
/// let response = client.get_daylight_savings_time(&request);
/// ```
pub struct DSTListRequest {
    year: Option<i32>,
    country: Option<String>,
    lang: Option<String>,
    listplaces: Option<u8>,
    onlydst: Option<u8>,
    timechanges: Option<u8>,
    verbosetime: Option<u8>,
}

impl DSTListRequest {
    /// Start building a new request.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set year to query for.
    pub fn set_year(mut self, year: i32) -> Self {
        self.year.insert(year);

        self
    }

    /// Set country to query for.
    pub fn set_country(mut self, country: impl Into<String>) -> Self {
        self.country.insert(country.into());

        self
    }

    ///  Set request language.
    pub fn set_lang(mut self, lang: impl Into<String>) -> Self {
        self.lang.insert(lang.into());

        self
    }

    /// Toggle whether to include individual places belonging to each country.
    pub fn set_listplaces(mut self, enable: bool) -> Self {
        self.listplaces.insert(enable.into());

        self
    }

    /// Toggle whether to only include places which observe DST.
    pub fn set_onlydst(mut self, enable: bool) -> Self {
        self.onlydst.insert(enable.into());

        self
    }

    /// Toggle whether to include time change information.
    pub fn set_timechanges(mut self, enable: bool) -> Self {
        self.timechanges.insert(enable.into());

        self
    }

    /// Toggle whether to include verbose time stamps.
    pub fn set_verbosetime(mut self, enable: bool) -> Self {
        self.verbosetime.insert(enable.into());

        self
    }
}
