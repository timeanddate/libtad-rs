use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
/// All valid moon phases.
pub enum MoonPhase {
    /// New moon.
    NewMoon,

    /// Waxing crescent.
    WaxingCrescent,

    /// Moon in first quarter.
    FirstQuarter,

    /// Waxing gibbous moon.
    WaxingGibbous,

    /// Full moon.
    FullMoon,

    /// Waning gibbous moon.
    WaningGibbous,

    /// Moon in third quarter.
    ThirdQuarter,

    /// Waning crescent moon.
    WaningCrescent,
}
