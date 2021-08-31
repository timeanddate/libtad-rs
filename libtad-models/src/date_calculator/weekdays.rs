use super::IncludedExcluded;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// The spread of excluded or included weekdays in includeddays.
pub struct Weekdays {
    /// Specifies whether or not the weekdays counted were part of an included or excluded filter.
    pub r#type: IncludedExcluded,

    /// How many days in total have been counted.
    pub count: i32,

    /// Count for Mondays.
    pub mon: i32,

    /// Count for Tuesdays.
    pub tue: i32,

    /// Count for Wednesdays.
    pub wed: i32,

    /// Count for Thursdays.
    pub thu: i32,

    /// Count for Fridays.
    pub fri: i32,

    /// Count for Saturdays.
    pub sat: i32,

    /// Count for Sundays.
    pub sun: i32,
}
