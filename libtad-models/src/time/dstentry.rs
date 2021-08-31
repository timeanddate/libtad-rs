use super::{TimeChange, TimeZone};
use crate::places::Region;
use serde::Deserialize;

#[derive(Deserialize)]
/// DST information about a region.
pub struct DSTEntry {
    /// The geographical region where this information is valid.
    /// Contains country, a textual description of the region and the name of the biggest place.
    pub region: Region,

    /// Information about the standard time zone.
    pub stdtimezone: TimeZone,

    /// Information about the daylight savings time zone. Please note that if the region is on
    /// daylight savings time for the whole year, this information will be returned in the
    /// stdtimezone element. Additionally, the special element will be set to allyear.
    pub dsttimezone: Option<TimeZone>,

    /// Indicates if the region does not observe DST at all, or is on DST all year long.
    pub special: Option<DSTEntrySpecial>,

    /// Starting date of daylight savings time.
    pub dststart: Option<String>,

    /// Ending date of daylight savings time.
    pub dstend: Option<String>,

    /// Time changes (daylight savings time).
    pub timechanges: Option<Vec<TimeChange>>,
}

#[derive(Deserialize)]
/// Indicates if the region does not observe DST at all, or is on DST all year long.
pub struct DSTEntrySpecial {
    /// The type of digression.
    pub r#type: DSTEntrySpecialType,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
/// Special type of DST observation.
pub enum DSTEntrySpecialType {
    /// DST is not observed at all.
    NoDST,

    /// DST is observed all year.
    AllYear,
}
