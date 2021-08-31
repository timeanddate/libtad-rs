use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
/// Choose a set of types or days you want to filter on.
pub enum BusinessDaysFilterType {
    /// Include or exclude Mondays.
    Mon,

    /// Include or exclude Tuesdays.
    Tue,

    /// Include or exclude Wednesdays.
    Wed,

    /// Include or exclude Thursdays.
    Thu,

    /// Include or exclude Fridays.
    Fri,

    /// Include or exclude Saturdays.
    Sat,

    /// Include or exclude Sundays.
    Sun,

    /// Include everyting (only supported if include parameter is set to true).
    All,

    /// Include or exclude weekends.
    Weekend,

    /// Include or exclude holidays.
    Holidays,

    /// Include or exclude weekends and holidays comnined.
    WeekendHolidays,

    /// Include or exclude nothing.
    None,
}
