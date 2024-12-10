//! CSS color value validation.

pub use csscolorparser::{Color, ParseColorError};
use serde::{Deserialize, Serialize};

/// A valid CSS color value, as defined in W3C's
/// [CSS Color Module Level 4](https://www.w3.org/TR/css-color-4/).
///
/// This struct is used to ensure that a given string is a valid
/// CSS color value. It uses the [`csscolorparser`] crate to parse
/// the color string. The color value is stored as the string
/// that was fed during construction.
///
/// This requires the value to be parsed both on construction of a
/// [`CssColor`] instance, as well as later when the value is
/// converted to a [`Color`] by the consumer of this API, which may
/// not be desirable. In that case, consider using a raw string
/// instead, and parse it manually using the [`csscolorparser`] crate.
///
/// See the [`csscolorparser`] crate for more information on CSS
/// color parsing.
/// # Examples
/// A [`CssColor`] can be created from any string that is a valid
/// CSS color value. Instances can be converted into a [`Color`]
/// using the [`into_color`](CssColor::into_color) method.
/// ```
/// # use chipbox_common::css::color::CssColor;
/// let crimson_named = CssColor::new("crimson").unwrap();
/// let crimson_hex = CssColor::new("#dc143c").unwrap();
/// assert_eq!(crimson_named.into_color(), crimson_hex.into_color());
/// ```
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, derive_more::AsRef)]
pub struct CssColor(String);

impl CssColor {
    /// Creates a new [`CssColor`] instance from a given string.
    ///
    /// This function asserts that the provided string is a valid
    /// CSS color value. If the string is not a valid CSS color
    /// value, a [`ParseColorError`] will be returned.
    ///
    /// # Returns
    /// A `Result` containing the [`CssColor`] instance if the
    /// string is a valid CSS color value, or a `ParseColorError`
    /// if it isn't.
    pub fn new<S>(color: S) -> Result<Self, ParseColorError>
    where
        S: Into<String> + AsRef<str>,
    {
        csscolorparser::parse(color.as_ref()).map(|_| Self(color.into()))
    }

    /// Converts the [`CssColor`] into a [`Color`] instance.
    ///
    /// This function will panic if the string is not a valid
    /// CSS color value. This in general should never happen.
    /// The string is validated in the [`new`](CssColor::new)
    /// function, and the default constructor is not usable,
    /// due to the inner string field being private.
    pub fn into_color(self) -> Color {
        csscolorparser::parse(self.as_ref()).expect("valid css color")
    }
}
