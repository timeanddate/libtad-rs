use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// A full name.
pub struct Name {
    /// First name.
    pub first: Option<String>,

    /// Middle name.
    pub middle: Option<String>,

    /// Last name.
    pub last: Option<String>,
}
