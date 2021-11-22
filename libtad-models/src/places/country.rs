use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Information about a country.
pub struct Country {
    /// The ISO 3166-1-alpha-2 country code.
    /// See <https://dev.timeanddate.com/docs/type-iso#isoCountry>
    pub id: String,

    /// Full name of the country.
    pub name: String,
}
