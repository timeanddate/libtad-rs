use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
/// Set if the service should do an addition or subtraction of the specified days.
pub enum BusinessDaysOperatorType {
    /// Uses the number specified in parameter to add the days from a start date.
    Add,

    /// Uses the number specified in parameter to subtract the days from a start date.
    Subtract,
}
