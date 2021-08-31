use crate::http::HTTPError;
use hmac::crypto_mac::InvalidKeyLength;
use serde::Deserialize;

#[derive(Debug)]
/// Internal library errors.
pub enum Error {
    /// Error returned from the HTTP client.
    Http(HTTPError),

    /// Error when generating signature.
    Signature(InvalidKeyLength),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Http(e) => write!(f, "HTTP Error: {}", e),
            Self::Signature(e) => write!(f, "Signature Error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Http(e) => Some(e),
            _ => None,
        }
    }
}

impl From<HTTPError> for Error {
    fn from(e: HTTPError) -> Self {
        Self::Http(e)
    }
}

impl From<InvalidKeyLength> for Error {
    fn from(e: InvalidKeyLength) -> Self {
        Self::Signature(e)
    }
}

#[derive(Debug, Deserialize)]
/// Error returned from the API.
pub struct ApiError {
    /// List of API errors.
    #[serde(deserialize_with = "custom_deserialize::string_or_vec")]
    pub errors: Vec<String>,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "API Error: {}", self.errors.join(", "))
    }
}

impl std::error::Error for ApiError {}

mod custom_deserialize {
    use serde::de::{self, value, Deserialize, Deserializer, SeqAccess, Visitor};

    pub fn string_or_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StringOrVec;

        impl<'de> Visitor<'de> for StringOrVec {
            type Value = Vec<String>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("string or list of strings")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(vec![s.to_owned()])
            }

            fn visit_seq<S>(self, seq: S) -> Result<Self::Value, S::Error>
            where
                S: SeqAccess<'de>,
            {
                Deserialize::deserialize(value::SeqAccessDeserializer::new(seq))
            }
        }

        deserializer.deserialize_any(StringOrVec)
    }
}
