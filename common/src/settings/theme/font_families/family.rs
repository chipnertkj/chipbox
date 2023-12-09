use cssparser::ToCss as _;
mod custom;
mod generic;

pub use crate::settings::theme::global::Global;
pub use custom::Custom;
pub use generic::{
    Generic, CURSIVE_STR, EMOJI_STR, FANGSONG_STR, FANTASY_STR, MATH_STR,
    MONOSPACE_STR, SANS_SERIF_STR, SERIF_STR, SYSTEM_UI_STR, UI_MONOSPACE_STR,
    UI_ROUNDED_STR, UI_SANS_SERIF_STR, UI_SERIF_STR,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum FontFamily {
    Custom(Custom),
    Generic(Generic),
    Global(Global),
}

impl<'a> TryFrom<&'a str> for FontFamily {
    type Error = cssparser::ParseError<'a, ()>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Custom::try_from(value)
            .map(Self::Custom)
            .or_else(|_| Generic::try_from(value).map(Self::Generic))
            .or_else(|_| Global::try_from(value).map(Self::Global))
    }
}

impl cssparser::ToCss for FontFamily {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        match self {
            FontFamily::Global(global) => global.to_css(dest),
            FontFamily::Generic(generic) => generic.to_css(dest),
            FontFamily::Custom(custom) => custom.to_css(dest),
        }
    }
}

impl std::fmt::Display for FontFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_css(f)
    }
}

impl AsRef<str> for FontFamily {
    fn as_ref(&self) -> &str {
        match self {
            FontFamily::Global(global) => global.as_ref(),
            FontFamily::Generic(generic) => generic.as_ref(),
            FontFamily::Custom(custom) => custom.as_ref(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn family_generic() {
        let input = "sans-serif";
        let font_family: FontFamily = input.try_into().unwrap();
        assert_eq!(input, font_family.as_ref());
        assert_eq!(font_family, FontFamily::Generic(Generic::SansSerif));
    }

    #[test]
    fn family_custom() {
        let input = "\"Lato\"";
        let font_family: FontFamily = input.try_into().unwrap();
        assert_eq!(input, font_family.as_ref());
    }

    #[test]
    fn family_global() {
        let input = "inherit";
        let font_family: FontFamily = input.try_into().unwrap();
        assert_eq!(input, font_family.as_ref());
        assert_eq!(font_family, FontFamily::Global(Global::Inherit));
    }
}
