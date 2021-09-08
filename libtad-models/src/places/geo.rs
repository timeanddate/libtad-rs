use super::Country;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Geographical information about a location.
pub struct Geo {
    /// The name of the location.
    pub name: String,

    /// The state of the location within the country (only if applicable).
    pub state: Option<String>,

    /// Country of the location.
    pub country: Country,

    /// Geographical latitude of the location.
    #[serde(
        deserialize_with = "custom_deserialize::float_or_empty_string",
        default
    )]
    pub latitude: Option<f32>,

    /// Geographical longitude of the location.
    #[serde(
        deserialize_with = "custom_deserialize::float_or_empty_string",
        default
    )]
    pub longitude: Option<f32>,
}

mod custom_deserialize {
    use serde::de::{self, Deserializer, Visitor};

    pub fn float_or_empty_string<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FloatOrString;

        impl<'de> Visitor<'de> for FloatOrString {
            type Value = Option<f32>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("float or empty string")
            }

            fn visit_str<E>(self, _: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(None)
            }

            fn visit_f64<E>(self, s: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Some(s as f32))
            }
        }
        deserializer.deserialize_any(FloatOrString)
    }
}
