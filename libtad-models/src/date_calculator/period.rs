use super::{BusinessHoliday, Weekdays};
use crate::time::Time;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Calculated results for a requested period.
pub struct Period {
    /// Number of days calculated.
    pub includeddays: i32,

    /// Number of calendar days in calculated period.
    pub calendardays: i32,

    /// Number of days which was skipped in the calculated period.
    pub skippeddays: i32,

    /// The date the calculation started from.
    pub startdate: Time,

    /// The date the calculation ended on.
    pub enddate: Time,

    /// The spread of excluded or included weekdays in includdeddays.
    pub weekdays: Weekdays,

    /// Holidays which occur in the requested period.
    pub holidays: BusinessHoliday,
}
