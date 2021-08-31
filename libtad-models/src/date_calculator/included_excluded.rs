use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
/// Whether or not the weekdays counted were part of an included or excluded filter.
pub enum IncludedExcluded {
    /// Included.
    Included,

    /// Excluded.
    Excluded,
}
