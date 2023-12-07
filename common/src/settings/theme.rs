mod color;
mod dimension;
mod font_families;
mod font_weight;
mod global;
mod themes;
mod user;

pub use color::Color;
pub use dimension::Dimension;
pub use font_families::FontFamilies;
pub use themes::{get, ThemeSelector};
pub use user::UserThemes;

use self::font_weight::FontWeight;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Theme {
    pub name: String,
    pub fonts: FontTheme,
    pub text: TextTheme,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct FontTheme {
    pub family_sans: FontFamilies,
    pub family_mono: FontFamilies,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct TextProps {
    pub color: Color,
    pub font_size: Dimension,
    pub font_weight: FontWeight,
    pub line_height: Dimension,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct TextTheme {
    pub primary: TextProps,
    pub secondary: TextProps,
    pub tertiary: TextProps,
}
