use libtad_models::holidays::Holiday;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Holidays API response.
pub struct HolidaysResponse {
    /// List of returned holidays.
    pub holidays: Vec<Holiday>,
}
