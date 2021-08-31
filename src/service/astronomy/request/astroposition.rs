use crate::models::{astronomy::AstronomyObjectType, time::DateTime};
use crate::service::{ProvidedArgument, RequiredArgument};
use serde::Serialize;

macro_rules! return_type {
    ($self:ident) => {
        AstroPositionRequest {
            object: $self.object,
            placeid: $self.placeid,
            interval: $self.interval,
            localtime: $self.localtime,
            utctime: $self.utctime,
            isotime: $self.isotime,
            lang: $self.lang,
            radius: $self.radius,
            _a: Default::default(),
            _b: Default::default(),
            _c: Default::default(),
        }
    };
}

#[derive(Default, Serialize)]
/// Astro Position API request.
///
/// Request is validated at compile time when supplied to the client.
///
/// Example:
/// ```
/// use libtad_rs::{
///     ServiceClient,
///     service::astronomy::AstroPositionRequest,
///     models::{
///         astronomy::AstronomyObjectType,
///         time::DateTime,
///     },
/// };
///
/// let client = ServiceClient::new("access_key".into(), "secret_key".into());
/// let request = AstroPositionRequest::new()
///     .with_object(AstronomyObjectType::Sun)
///     .with_placeid("3")
///     .with_interval(DateTime::from("2021-08-18"));
///
/// let response = client.get_astro_position(&request);
/// ```
pub struct AstroPositionRequest<A = ProvidedArgument, B = ProvidedArgument, C = ProvidedArgument> {
    object: Vec<AstronomyObjectType>,
    placeid: Vec<String>,
    interval: Vec<DateTime>,
    localtime: Option<u8>,
    utctime: Option<u8>,
    isotime: Option<u8>,
    lang: Option<String>,
    radius: Option<i32>,
    _a: std::marker::PhantomData<A>,
    _b: std::marker::PhantomData<B>,
    _c: std::marker::PhantomData<C>,
}

impl AstroPositionRequest {
    /// Start building a new request.
    pub fn new() -> AstroPositionRequest<RequiredArgument, RequiredArgument, RequiredArgument> {
        Default::default()
    }
}

impl<A, B, C> AstroPositionRequest<A, B, C> {
    /// Set a list of objects for the request.
    pub fn set_object(
        mut self,
        object: Vec<AstronomyObjectType>,
    ) -> AstroPositionRequest<ProvidedArgument, B, C> {
        self.object = object;

        return_type!(self)
    }

    /// Add an object to the request.
    pub fn with_object(
        mut self,
        object: AstronomyObjectType,
    ) -> AstroPositionRequest<ProvidedArgument, B, C> {
        self.object.push(object);

        return_type!(self)
    }

    /// Set a list of placeids for the request.
    pub fn set_placeid(
        mut self,
        placeid: Vec<impl Into<String>>,
    ) -> AstroPositionRequest<A, ProvidedArgument, C> {
        self.placeid = placeid.into_iter().map(Into::into).collect();

        return_type!(self)
    }

    /// Add a placeid to the request.
    pub fn with_placeid(
        mut self,
        placeid: impl Into<String>,
    ) -> AstroPositionRequest<A, ProvidedArgument, C> {
        self.placeid.push(placeid.into());

        return_type!(self)
    }

    /// Set a list of intervals for the request.
    pub fn set_interval(
        mut self,
        interval: Vec<DateTime>,
    ) -> AstroPositionRequest<A, B, ProvidedArgument> {
        self.interval = interval;

        return_type!(self)
    }

    /// Add an interval to the request.
    pub fn with_interval(
        mut self,
        interval: DateTime,
    ) -> AstroPositionRequest<A, B, ProvidedArgument> {
        self.interval.push(interval);

        return_type!(self)
    }

    /// Toggle whether intervals should be considered local time or UTC time.
    pub fn set_localtime(mut self, enable: bool) -> Self {
        self.localtime.insert(enable.into());

        self
    }

    /// Toggle whether to add time stamps in UTC to all events.
    pub fn set_utctime(mut self, enable: bool) -> Self {
        self.utctime.insert(enable.into());

        self
    }

    /// Toggle whether to add time stamps in local time to all events.
    pub fn set_isotime(mut self, enable: bool) -> Self {
        self.isotime.insert(enable.into());

        self
    }

    /// Set the request language for the request.
    pub fn set_lang(mut self, lang: impl Into<String>) -> Self {
        self.lang.insert(lang.into());

        self
    }

    /// Set search radius for translating coordinates to locations.
    pub fn set_radius(mut self, radius: i32) -> Self {
        self.radius.insert(radius);

        self
    }
}
