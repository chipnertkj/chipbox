use cssparser::ToCss as _;

use self::absolute_weight::AbsoluteWeight;
use self::relative_weight::RelativeWeight;
use super::global::Global;

pub use absolute_weight::{BOLD_STR, NORMAL_STR};
pub use relative_weight::{BOLDER_STR, LIGHTER_STR};

mod absolute_weight;
mod relative_weight;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum FontWeight {
    Absolute(AbsoluteWeight),
    Relative(RelativeWeight),

    Global(Global),
}

impl<'a> TryFrom<&'a str> for FontWeight {
    type Error = cssparser::ParseError<'a, ()>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        AbsoluteWeight::try_from(value)
            .map(Self::Absolute)
            .or_else(|_| RelativeWeight::try_from(value).map(Self::Relative))
            .or_else(|_| Global::try_from(value).map(Self::Global))
    }
}

impl cssparser::ToCss for FontWeight {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        match self {
            Self::Absolute(weight) => weight.to_css(dest),
            Self::Relative(weight) => weight.to_css(dest),
            Self::Global(global) => global.to_css(dest),
        }
    }
}

impl std::fmt::Display for FontWeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_css(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absolute() {
        let input = "bold";
        let font_weight: FontWeight = input.try_into().unwrap();
        assert_eq!(
            input,
            font_weight
                .to_string()
                .as_str()
        );
        assert_eq!(font_weight, FontWeight::Absolute(AbsoluteWeight::Bold));
    }

    #[test]
    fn relative() {
        let input = "bolder";
        let font_weight: FontWeight = input.try_into().unwrap();
        assert_eq!(
            input,
            font_weight
                .to_string()
                .as_str()
        );
        assert_eq!(font_weight, FontWeight::Relative(RelativeWeight::Bolder));
    }

    #[test]
    fn global() {
        let input = "inherit";
        let font_weight: FontWeight = input.try_into().unwrap();
        assert_eq!(
            input,
            font_weight
                .to_string()
                .as_str()
        );
        assert_eq!(font_weight, FontWeight::Global(Global::Inherit));
    }

    #[test]
    fn invalid() {
        let input = "invalid";
        let font_weight: Result<FontWeight, _> = input.try_into();
        assert!(font_weight.is_err());
    }

    #[test]
    fn unexhausted() {
        let input = format!("{bolder} invalid", bolder = BOLDER_STR);
        let font_weight: Result<FontWeight, _> = input.as_str().try_into();
        assert!(font_weight.is_err());
    }
}
