use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// A text in a specific language.
pub struct Text {
    /// ISO 639 language code for this text.
    ///
    /// Example: en
    pub lang: String,

    /// The actual text.
    pub text: String,
}
