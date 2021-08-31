use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
/// All valid holiday types.
pub enum HolidayType {
    /// Combinations of all known types (except fun).
    All,

    /// Default holiday set: federal, federallocal, obs1, weekday.
    Default,

    /// Default set depending on country.
    /// For most countries, this is the same as default. However,
    /// for some countries it makes sense to add further types - this type
    /// accounts for this case. Currently this only affects the UK:
    /// local holidays are added as well. This is to include days that
    /// are only valid in one of the countries - e.g. Jan 2 is a holiday only for Scotland.
    CountryDefault,

    /// Important (obs1), common (obs2) and other observances (obs3).
    Obs,

    /// All religious holidays: buddhism, christian, hebrew, hinduism, muslm, orthodox.
    Religious,

    /// Some countries (e.g. Sweden) have days which are de facto treated as official holidays,
    /// even if there's no legal regulation.
    Defacto,

    /// Federal/national holidays.
    Federal,

    /// Common local holidays.
    FederalLocal,

    /// Flag days.
    FlagDay,

    /// Half day holidays (only afternoon off). These days can be half holidays either by law, or
    /// being de facto half day holidays (e.g. Sweden).
    HalfDay,

    /// Local holidays.
    Local,

    /// Local observances.
    Local2,

    ///Important observances.
    Obs1,

    /// Common observances.
    Obs2,

    /// Other observances.
    Obs3,

    /// Optional holiday.
    /// Employment and holiday laws in certain countries allow employees to choose a limited number
    /// of holidays from a list of holidays. Some employees may choose to take the day off on these
    /// days, however, most offices and businesses remain open.
    Optional,

    /// Normal working days.
    /// In some cases, working days are declared non-working days in order to form a longer period
    /// of consecutive non-working days. In exchange, weekend days become normal working days.
    Weekday,

    /// Buddhist holidays.
    Buddhism,

    /// Christian holidays.
    Christian,

    /// Hebrew holidays.
    Hebrew,

    /// Hindu holidays.
    Hinduism,

    /// Muslim holidays.
    Muslim,

    /// Orthodox holidays.
    Orthodox,

    /// Religious holidays, not covered by other types.
    OtherReligion,

    /// Seasons (equinoxes and solstices).
    Seasons,

    /// Sport events.
    Sport,

    /// Time zone events - daylight savings time start and end.
    TZ,

    /// United Nations days.
    UN,

    /// Worldwide observances.
    World,

    /// Fun, Wacky and Trivial holidays.
    Fun,
}
