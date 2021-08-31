use crate::service::{ProvidedArgument, RequiredArgument};
use libtad_models::{date_calculator::BusinessDaysFilterType, time::DateTime};
use serde::Serialize;

macro_rules! return_type {
    ($self:ident) => {
        BusinessDurationRequest {
            placeid: $self.placeid,
            country: $self.country,
            state: $self.state,
            startdt: $self.startdt,
            enddt: $self.enddt,
            include: $self.include,
            filter: $self.filter,
            includelastdate: $self.includelastdate,
            lang: $self.lang,
            verbosetime: $self.verbosetime,
            _a: Default::default(),
            _b: Default::default(),
            _c: Default::default(),
        }
    };
}

#[derive(Default, Serialize)]
/// Business Duration API request.
///
/// Request is validated at compile time when supplied to the client.
///
/// Example:
/// ```
/// use libtad_rs::{
///     ServiceClient,
///     service::date_calculator::BusinessDurationRequest,
///     models::time::DateTime,
/// };
///
/// let client = ServiceClient::new("access_key".into(), "secret_key".into());
/// let request = BusinessDurationRequest::new()
///     .set_country("dk")
///     .set_startdt(DateTime::from("2021-08-12"))
///     .set_enddt(DateTime::from("2021-08-20"));
///
/// let response = client.calculate_business_duration(&request);
/// ```
pub struct BusinessDurationRequest<A = ProvidedArgument, B = ProvidedArgument, C = ProvidedArgument>
{
    placeid: Option<String>,
    country: Option<String>,
    state: Option<String>,
    startdt: DateTime,
    enddt: DateTime,
    include: Option<u8>,
    filter: Option<Vec<BusinessDaysFilterType>>,
    includelastdate: Option<u8>,
    lang: Option<String>,
    verbosetime: Option<u8>,
    _a: std::marker::PhantomData<A>,
    _b: std::marker::PhantomData<B>,
    _c: std::marker::PhantomData<C>,
}

impl BusinessDurationRequest {
    /// Start building a new request.
    pub fn new() -> BusinessDurationRequest<RequiredArgument, RequiredArgument, RequiredArgument> {
        Default::default()
    }
}

impl<A, B, C> BusinessDurationRequest<A, B, C> {
    /// Set placeid to calculate for.
    pub fn set_placeid(
        mut self,
        placeid: impl Into<String>,
    ) -> BusinessDurationRequest<ProvidedArgument, B, C> {
        self.placeid.insert(placeid.into());

        return_type!(self)
    }

    /// Set country to calculate for.
    pub fn set_country(
        mut self,
        country: impl Into<String>,
    ) -> BusinessDurationRequest<ProvidedArgument, B, C> {
        self.country.insert(country.into());

        return_type!(self)
    }

    /// Set state for the given country.
    pub fn set_state(mut self, state: impl Into<String>) -> Self {
        self.state.insert(state.into());

        self
    }

    /// Set start date for the request.
    pub fn set_startdt(
        mut self,
        startdt: DateTime,
    ) -> BusinessDurationRequest<A, ProvidedArgument, C> {
        self.startdt = startdt;

        return_type!(self)
    }

    /// Set end date for the request.
    pub fn set_enddt(mut self, enddt: DateTime) -> BusinessDurationRequest<A, B, ProvidedArgument> {
        self.enddt = enddt;

        return_type!(self)
    }

    /// Toggle whether the result should be calculated by including instead of excluding days.
    pub fn set_include(mut self, enable: bool) -> Self {
        self.include.insert(enable.into());

        self
    }

    /// Add a filter to the request.
    pub fn with_filter(mut self, filter: BusinessDaysFilterType) -> Self {
        if let Some(ref mut filters) = self.filter {
            filters.push(filter);
        } else {
            self.filter.insert(vec![filter]);
        }

        self
    }

    /// Set whether or not the last date should be counted in the result.
    pub fn set_includelastdate(mut self, enable: bool) -> Self {
        self.includelastdate.insert(enable.into());

        self
    }

    /// Set request language.
    pub fn set_lang(mut self, lang: impl Into<String>) -> Self {
        self.lang.insert(lang.into());

        self
    }

    /// Toggle whether to include verbose time stamps.
    pub fn set_verbosetime(mut self, enable: bool) -> Self {
        self.verbosetime.insert(enable.into());

        self
    }
}
