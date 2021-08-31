#![warn(missing_docs)]
#![allow(clippy::module_inception)]

//! Types and models in use by Time and Date API services.

mod text;
pub use text::Text;

/// Astronomy models.
pub mod astronomy;

/// Date Calculator models.
pub mod date_calculator;

/// Holiday models.
pub mod holidays;

/// On This Day models.
pub mod onthisday;

/// Place models.
pub mod places;

/// Time models.
pub mod time;
