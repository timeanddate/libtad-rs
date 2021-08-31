use crate::models::{
    astronomy::{AstronomyEventClass, AstronomyObjectType},
    time::DateTime,
};
use crate::service::{ProvidedArgument, RequiredArgument};
use serde::Serialize;

macro_rules! return_type {
    ($self:ident) => {
        AstroEventRequest {
            object: $self.object,
            placeid: $self.placeid,
            startdt: $self.startdt,
            enddt: $self.enddt,
            types: $self.types,
            geo: $self.geo,
            isotime: $self.isotime,
            lang: $self.lang,
            radius: $self.radius,
            utctime: $self.utctime,
            _a: Default::default(),
            _b: Default::default(),
            _c: Default::default(),
        }
    };
}

#[derive(Default, Serialize)]
/// Astro Event API request.
///
/// Request is validated at compile time when supplied to the client.
///
/// Example:
/// ```
/// use libtad_rs::{
///     ServiceClient,
///     service::astronomy::AstroEventRequest,
///     models::{
///         astronomy::AstronomyObjectType,
///         time::DateTime,
///     },
/// };
///
/// let client = ServiceClient::new("access_key".into(), "secret_key".into());
/// let request = AstroEventRequest::new()
///     .with_object(AstronomyObjectType::Sun)
///     .with_placeid("3")
///     .set_startdt(DateTime::from("2021-08-18"));
///
/// let response = client.get_astro_events(&request);
/// ```
pub struct AstroEventRequest<A = ProvidedArgument, B = ProvidedArgument, C = ProvidedArgument> {
    object: Vec<AstronomyObjectType>,
    placeid: Vec<String>,
    startdt: DateTime,
    enddt: Option<DateTime>,
    types: Option<Vec<AstronomyEventClass>>,
    geo: Option<u8>,
    isotime: Option<u8>,
    lang: Option<String>,
    radius: Option<i32>,
    utctime: Option<u8>,
    _a: std::marker::PhantomData<A>,
    _b: std::marker::PhantomData<B>,
    _c: std::marker::PhantomData<C>,
}

impl AstroEventRequest {
    /// Start building a new request.
    pub fn new() -> AstroEventRequest<RequiredArgument, RequiredArgument, RequiredArgument> {
        Default::default()
    }
}

impl<A, B, C> AstroEventRequest<A, B, C> {
    /// Set a list of objects for the request.
    pub fn set_object(
        mut self,
        object: Vec<AstronomyObjectType>,
    ) -> AstroEventRequest<ProvidedArgument, B, C> {
        self.object = object;

        return_type!(self)
    }

    /// Add an object to the request.
    pub fn with_object(
        mut self,
        object: AstronomyObjectType,
    ) -> AstroEventRequest<ProvidedArgument, B, C> {
        self.object.push(object);

        return_type!(self)
    }

    /// Set a list of placeids for the request.
    pub fn set_placeid(
        mut self,
        placeid: Vec<impl Into<String>>,
    ) -> AstroEventRequest<A, ProvidedArgument, C> {
        self.placeid = placeid.into_iter().map(Into::into).collect();

        return_type!(self)
    }

    /// Add a placeid to the request.
    pub fn with_placeid(
        mut self,
        placeid: impl Into<String>,
    ) -> AstroEventRequest<A, ProvidedArgument, C> {
        self.placeid.push(placeid.into());

        return_type!(self)
    }

    /// Set the start date for the request.
    pub fn set_startdt(mut self, startdt: DateTime) -> AstroEventRequest<A, B, ProvidedArgument> {
        self.startdt = startdt;

        return_type!(self)
    }

    /// Set the end date for the request.
    pub fn set_enddt(mut self, enddt: DateTime) -> Self {
        self.enddt.insert(enddt);

        self
    }

    /// Add an event type to the request.
    pub fn with_type(mut self, event_type: AstronomyEventClass) -> Self {
        if let Some(ref mut types) = self.types {
            types.push(event_type);
        } else {
            self.types.insert(vec![event_type]);
        }

        self
    }

    /// Toggle whether to return longitude and latitude for the geo object.
    pub fn set_geo(mut self, enable: bool) -> Self {
        self.geo.insert(enable.into());

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

    /// Toggle whether to add time stamps in RequiredArgumentTC to all events.
    pub fn set_utctime(mut self, enable: bool) -> Self {
        self.utctime.insert(enable.into());

        self
    }
}
