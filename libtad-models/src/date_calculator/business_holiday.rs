use super::IncludedExcluded;
use crate::holidays::Holiday;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// A holiday event which occurs in a requested period.
pub struct BusinessHoliday {
    /// Specifying whether or not the holidays in the result list were included or excluded when
    /// queried.
    pub r#type: Option<IncludedExcluded>,

    /// The number of holidays in the results.
    pub count: Option<i32>,

    /// Holidays which occur in the requested period.
    pub list: Option<Vec<Holiday>>,
}
