use crate::service::{ProvidedArgument, RequiredArgument};
use serde::Serialize;

macro_rules! return_type {
    ($self:ident) => {
        TimeserviceRequest {
            placeid: $self.placeid,
            query: $self.query,
            qlimit: $self.qlimit,
            geo: $self.geo,
            lang: $self.lang,
            radius: $self.radius,
            sun: $self.sun,
            time: $self.time,
            timechanges: $self.timechanges,
            tz: $self.tz,
            verbosetime: $self.verbosetime,
            _a: Default::default(),
        }
    };
}

#[derive(Default, Serialize)]
/// Timeservice API request.
///
/// Request is validated at compile time when supplied to the client.
///
/// Example:
/// ```
/// use libtad_rs::{
///     ServiceClient,
///     service::time::TimeserviceRequest,
/// };
///
/// let client = ServiceClient::new("access_key".into(), "secret_key".into());
/// let request = TimeserviceRequest::new()
///     .set_query("new york")
///     .set_qlimit(2)
///     .set_sun(true);
///
/// let response = client.get_current_time(&request);
/// ```
pub struct TimeserviceRequest<A = ProvidedArgument> {
    placeid: Option<String>,
    query: Option<String>,
    qlimit: Option<u8>,
    geo: Option<u8>,
    lang: Option<String>,
    radius: Option<i32>,
    sun: Option<u8>,
    time: Option<u8>,
    timechanges: Option<u8>,
    tz: Option<u8>,
    verbosetime: Option<u8>,
    _a: std::marker::PhantomData<A>,
}

impl TimeserviceRequest {
    /// Start building a new request.
    pub fn new() -> TimeserviceRequest<RequiredArgument> {
        Default::default()
    }
}

impl<A> TimeserviceRequest<A> {
    /// Set location id to get current time for.
    pub fn set_placeid(
        mut self,
        placeid: impl Into<String>,
    ) -> TimeserviceRequest<ProvidedArgument> {
        self.placeid.insert(placeid.into());

        return_type!(self)
    }

    /// Set location query to get current time for.
    pub fn set_query(mut self, query: impl Into<String>) -> TimeserviceRequest<ProvidedArgument> {
        self.query.insert(query.into());

        return_type!(self)
    }

    /// Set the limit for query results to be returned.
    pub fn set_qlimit(mut self, qlimit: u8) -> Self {
        self.qlimit.insert(qlimit);

        self
    }

    /// Toggle whether to return longitude and latitude for the geo object.
    pub fn set_geo(mut self, enable: bool) -> Self {
        self.geo.insert(enable.into());

        self
    }

    /// Set the request language.
    pub fn set_lang(mut self, lang: impl Into<String>) -> Self {
        self.lang.insert(lang.into());

        self
    }

    /// Set search radius for translating coordinates to locations.
    pub fn set_radius(mut self, radius: i32) -> Self {
        self.radius.insert(radius);

        self
    }

    /// Toggle whether to include information about sunrise and sunset.
    pub fn set_sun(mut self, enable: bool) -> Self {
        self.sun.insert(enable.into());

        self
    }

    /// Toggle whether to include current time under the location object.
    pub fn set_time(mut self, enable: bool) -> Self {
        self.time.insert(enable.into());

        self
    }

    /// Toggle whether to include time change information.
    pub fn set_timechanges(mut self, enable: bool) -> Self {
        self.timechanges.insert(enable.into());

        self
    }

    /// Toggle whether to include time zone information.
    pub fn set_tz(mut self, enable: bool) -> Self {
        self.tz.insert(enable.into());

        self
    }

    /// Toggle whether to include verbose time stamps.
    pub fn set_verbosetime(mut self, enable: bool) -> Self {
        self.verbosetime.insert(enable.into());

        self
    }
}
