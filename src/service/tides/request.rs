use crate::models::time::DateTime;
use crate::service::{ProvidedArgument, RequiredArgument};
use serde::Serialize;

macro_rules! return_type {
    ($self:ident) => {
        TidesRequest {
            placeid: $self.placeid,
            onlyhighlow: $self.onlyhighlow,
            startdt: $self.startdt,
            enddt: $self.enddt,
            radius: $self.radius,
            subordinate: $self.subordinate,
            interval: $self.interval,
            localtime: $self.localtime,
            _a: Default::default(),
        }
    };
}

#[derive(Default, Serialize)]
/// Tides API request.
///
/// Request is validated when supplied to the client.
pub struct TidesRequest<A = ProvidedArgument> {
    placeid: Vec<String>,
    onlyhighlow: Option<u8>,
    startdt: Option<DateTime>,
    enddt: Option<DateTime>,
    radius: Option<i32>,
    subordinate: Option<u8>,
    interval: Option<i32>,
    localtime: Option<u8>,
    _a: std::marker::PhantomData<A>,
}

impl TidesRequest {
    /// Start building a new request.
    pub fn new() -> TidesRequest<RequiredArgument> {
        Default::default()
    }
}

impl<A> TidesRequest<A> {
    /// Add a placeid to the request.
    pub fn with_placeid(mut self, placeid: impl Into<String>) -> TidesRequest<ProvidedArgument> {
        self.placeid.push(placeid.into());

        return_type!(self)
    }

    /// Set whether to return every point per interval or just the highest and lowest points.
    pub fn set_onlyhighlow(mut self, enable: bool) -> Self {
        self.onlyhighlow.insert(enable.into());

        self
    }

    /// Set start of the requested time interval.
    pub fn set_startdt(mut self, startdt: DateTime) -> Self {
        self.startdt.insert(startdt);

        self
    }

    /// Set end of the requested time interval.
    pub fn set_enddt(mut self, enddt: DateTime) -> Self {
        self.enddt.insert(enddt);

        self
    }

    /// Set radius from the requested location to query for stations. Given in kilometers.
    pub fn set_radius(mut self, radius: i32) -> Self {
        self.radius.insert(radius);

        self
    }

    /// Toggle whether to resolve also subordinate stations or just reference stations.
    pub fn set_subordinate(mut self, enable: bool) -> Self {
        self.subordinate.insert(enable.into());

        self
    }

    /// Set how many minutes to calculate the range in. Supported: 5 min, 15, min, 30 min, 60 min.
    pub fn set_interval(mut self, interval: i32) -> Self {
        self.interval.insert(interval);

        self
    }

    /// Toggle whether input and output time stamps should be resolved to local time.
    pub fn set_localtime(mut self, enable: bool) -> Self {
        self.localtime.insert(enable.into());

        self
    }
}
