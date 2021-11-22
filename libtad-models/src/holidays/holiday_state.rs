use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// A holiday event in a specific state.
pub struct HolidayState {
    /// An ISO 3166-1 country or ISO 3166-2 country state code.
    ///
    /// See also: <https://dev.timeanddate.com/docs/type-iso#isoCountry>
    pub iso: String,

    /// Unique id of the state/subdivision.
    pub id: i32,

    /// Abbreviation of the state/subdivision.
    pub abbrev: String,

    /// Common name of the state/subdivision.
    pub name: String,

    /// Eventual exception if the holiday does not affect the whole state/subdivision.
    pub exception: Option<String>,
}
