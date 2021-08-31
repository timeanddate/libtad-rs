use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Information about a country.
pub struct Country {
    /// The ISO 3166-1-alpha-2 country code.
    /// See <https://services.timeanddate.com/api/doc/v3/bi01.html#ISO3166>
    pub id: String,

    /// Full name of the country.
    pub name: String,
}
