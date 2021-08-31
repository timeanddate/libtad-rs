use crate::service::{ProvidedArgument, RequiredArgument};
use libtad_models::{
    date_calculator::{BusinessDaysFilterType, BusinessDaysOperatorType},
    time::DateTime,
};
use serde::Serialize;

macro_rules! return_type {
    ($self:ident) => {
        BusinessDateRequest {
            placeid: $self.placeid,
            country: $self.country,
            state: $self.state,
            startdt: $self.startdt,
            days: $self.days,
            include: $self.include,
            filter: $self.filter,
            op: $self.op,
            repeat: $self.repeat,
            lang: $self.lang,
            verbosetime: $self.verbosetime,
            _a: Default::default(),
            _b: Default::default(),
            _c: Default::default(),
        }
    };
}

#[derive(Default, Serialize)]
/// Business Date API request.
///
/// Request is validated at compile time when supplied to the client.
///
/// Example:
/// ```
/// use libtad_rs::{
///     ServiceClient,
///     service::date_calculator::BusinessDateRequest,
///     models::time::DateTime,
/// };
///
/// let client = ServiceClient::new("access_key".into(), "secret_key".into());
/// let request = BusinessDateRequest::new()
///     .set_placeid("usa/chicago")
///     .set_startdt(DateTime::from("2021-09-10"))
///     .with_days(5);
///
/// let response = client.calculate_business_date(&request);
/// ```
pub struct BusinessDateRequest<A = ProvidedArgument, B = ProvidedArgument, C = ProvidedArgument> {
    placeid: Option<String>,
    country: Option<String>,
    state: Option<String>,
    startdt: Option<DateTime>,
    days: Vec<i32>,
    include: Option<u8>,
    filter: Option<Vec<BusinessDaysFilterType>>,
    op: Option<BusinessDaysOperatorType>,
    repeat: Option<i32>,
    lang: Option<String>,
    verbosetime: Option<u8>,
    _a: std::marker::PhantomData<A>,
    _b: std::marker::PhantomData<B>,
    _c: std::marker::PhantomData<C>,
}

impl BusinessDateRequest {
    /// Start building a new request.
    pub fn new() -> BusinessDateRequest<RequiredArgument, RequiredArgument, RequiredArgument> {
        Default::default()
    }
}

impl<A, B, C> BusinessDateRequest<A, B, C> {
    /// Set placeid to calculate for.
    pub fn set_placeid(
        mut self,
        placeid: impl Into<String>,
    ) -> BusinessDateRequest<ProvidedArgument, B, C> {
        self.placeid.insert(placeid.into());

        return_type!(self)
    }

    /// Set country to calculate for.
    pub fn set_country(
        mut self,
        country: impl Into<String>,
    ) -> BusinessDateRequest<ProvidedArgument, B, C> {
        self.country.insert(country.into());

        return_type!(self)
    }

    /// Set state for the given country.
    pub fn set_state(mut self, state: impl Into<String>) -> Self {
        self.state.insert(state.into());

        self
    }

    /// Set start date for the request.
    pub fn set_startdt(mut self, startdt: DateTime) -> BusinessDateRequest<A, ProvidedArgument, C> {
        self.startdt.insert(startdt);

        return_type!(self)
    }

    /// Set days to count by.
    pub fn set_days(mut self, days: Vec<i32>) -> BusinessDateRequest<A, B, ProvidedArgument> {
        self.days = days;

        return_type!(self)
    }

    /// Add an amount to count by.
    pub fn with_days(mut self, days: i32) -> BusinessDateRequest<A, B, ProvidedArgument> {
        self.days.push(days);

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

    /// Set the service to add days.
    pub fn set_addition(mut self) -> Self {
        self.op.insert(BusinessDaysOperatorType::Add);

        self
    }

    /// Set the service to subtract days.
    pub fn set_subtraction(mut self) -> Self {
        self.op.insert(BusinessDaysOperatorType::Subtract);

        self
    }

    /// Set how many times the calculation should be repeated.
    /// Only applicable when the days parameter has exactly one number.
    pub fn set_repeat(mut self, repeat: i32) -> Self {
        self.repeat.insert(repeat);

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
