use super::Name;
use crate::time::Time;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// A historical person.
pub struct Person {
    /// Identifier for the person.
    pub id: i32,

    /// Full name.
    pub name: Name,

    /// Date of birth.
    pub birthdate: Option<Time>,

    /// Date of death.
    pub deathdate: Option<Time>,

    /// Person categories.
    pub categories: Option<Vec<String>>,

    /// Person nationalities.
    pub nationalities: Option<Vec<String>>,
}
