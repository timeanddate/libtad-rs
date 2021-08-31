use crate::models::time::DateTime;
use crate::service::{ProvidedArgument, RequiredArgument};
use serde::Serialize;

macro_rules! return_type {
    ($self:ident) => {
        ConvertTimeRequest {
            fromid: $self.fromid,
            toid: $self.toid,
            iso: $self.iso,
            lang: $self.lang,
            radius: $self.radius,
            timechanges: $self.timechanges,
            tz: $self.tz,
            verbosetime: $self.verbosetime,
            _a: Default::default(),
            _b: Default::default(),
        }
    };
}

#[derive(Default, Serialize)]
/// ConvertTime API request.
///
/// Request is validated at compile time when supplied to the client.
///
/// Example:
/// ```
/// use libtad_rs::{
///     ServiceClient,
///     service::time::ConvertTimeRequest,
///     models::time::DateTime,
/// };
///
/// let client = ServiceClient::new("access_key".into(), "secret_key".into());
/// let request = ConvertTimeRequest::new()
///     .set_fromid("norway/oslo")
///     .set_datetime(DateTime::from("2021-04-05T16:45:02"))
///     .set_verbosetime(true);
///
/// let response = client.convert_time(&request);
/// ```
pub struct ConvertTimeRequest<A = ProvidedArgument, B = ProvidedArgument> {
    fromid: String,
    toid: Option<Vec<String>>,
    iso: DateTime,
    lang: Option<String>,
    radius: Option<i32>,
    timechanges: Option<u8>,
    tz: Option<u8>,
    verbosetime: Option<u8>,
    _a: std::marker::PhantomData<A>,
    _b: std::marker::PhantomData<B>,
}

impl ConvertTimeRequest {
    /// Start building a new request.
    pub fn new() -> ConvertTimeRequest<RequiredArgument, RequiredArgument> {
        Default::default()
    }
}

impl<A, B> ConvertTimeRequest<A, B> {
    /// Set location to convert from.
    pub fn set_fromid(
        mut self,
        fromid: impl Into<String>,
    ) -> ConvertTimeRequest<ProvidedArgument, B> {
        self.fromid = fromid.into();

        return_type!(self)
    }

    /// Add a location id to convert to.
    pub fn with_toid(mut self, toid: impl Into<String>) -> Self {
        if let Some(ref mut toids) = self.toid {
            toids.push(toid.into());
        } else {
            self.toid.insert(vec![toid.into()]);
        }

        self
    }

    /// Set date and time to convert.
    pub fn set_datetime(mut self, datetime: DateTime) -> ConvertTimeRequest<A, ProvidedArgument> {
        self.iso = datetime;

        return_type!(self)
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
