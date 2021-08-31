use crate::service::{ProvidedArgument, RequiredArgument};
use libtad_models::holidays::HolidayType;
use serde::Serialize;

macro_rules! return_type {
    ($self:ident) => {
        HolidaysRequest {
            country: $self.country,
            year: $self.year,
            lang: $self.lang,
            types: $self.types,
            tz: $self.tz,
            verbosetime: $self.verbosetime,
            _a: Default::default(),
            _b: Default::default(),
        }
    };
}

#[derive(Default, Serialize)]
/// Holidays API request.
///
/// Request is validated at compile time when supplied to the client.
///
/// Example:
/// ```
/// use libtad_rs::{
///     ServiceClient,
///     service::holidays::HolidaysRequest,
/// };
///
/// let client = ServiceClient::new("access_key".into(), "secret_key".into());
/// let request = HolidaysRequest::new()
///     .with_country("no")
///     .with_country("us")
///     .set_year(2022);
///
/// let response = client.get_holidays(&request);
/// ```
pub struct HolidaysRequest<A = ProvidedArgument, B = ProvidedArgument> {
    country: Vec<String>,
    year: u16,
    lang: Option<Vec<String>>,
    types: Option<Vec<HolidayType>>,
    tz: Option<u8>,
    verbosetime: Option<u8>,
    _a: std::marker::PhantomData<A>,
    _b: std::marker::PhantomData<B>,
}

impl HolidaysRequest {
    /// Start building a new request.
    pub fn new() -> HolidaysRequest<RequiredArgument, RequiredArgument> {
        Default::default()
    }
}

impl<A, B> HolidaysRequest<A, B> {
    /// Add a country to the request.
    pub fn with_country(
        mut self,
        country: impl Into<String>,
    ) -> HolidaysRequest<ProvidedArgument, B> {
        self.country.push(country.into());

        return_type!(self)
    }

    /// Set the request year for the request.
    pub fn set_year(mut self, year: u16) -> HolidaysRequest<A, ProvidedArgument> {
        self.year = year;

        return_type!(self)
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

    /// Add a holiday type to the request.
    pub fn with_type(mut self, holiday_type: HolidayType) -> Self {
        if let Some(ref mut types) = self.types {
            types.push(holiday_type);
        } else {
            self.types.insert(vec![holiday_type]);
        }

        self
    }

    /// Toggle whether to request time zone information.
    pub fn set_tz(mut self, enable: bool) -> Self {
        self.tz.insert(enable.into());

        self
    }

    /// Toggle whether to request verbose time.
    pub fn set_verbose_time(mut self, enable: bool) -> Self {
        self.verbosetime.insert(enable.into());

        self
    }
}
