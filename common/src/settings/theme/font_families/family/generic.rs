use cssparser::ToCss as _;

pub const SERIF_STR: &str = "serif";
pub const SANS_SERIF_STR: &str = "sans-serif";
pub const MONOSPACE_STR: &str = "monospace";
pub const CURSIVE_STR: &str = "cursive";
pub const FANTASY_STR: &str = "fantasy";
pub const SYSTEM_UI_STR: &str = "system-ui";
pub const UI_SERIF_STR: &str = "ui-serif";
pub const UI_SANS_SERIF_STR: &str = "ui-sans-serif";
pub const UI_MONOSPACE_STR: &str = "ui-monospace";
pub const UI_ROUNDED_STR: &str = "ui-rounded";
pub const EMOJI_STR: &str = "emoji";
pub const MATH_STR: &str = "math";
pub const FANGSONG_STR: &str = "fangsong";

/// https://developer.mozilla.org/en-US/docs/Web/CSS/font-family#syntax
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Generic {
    Serif,
    SansSerif,
    Monospace,
    Cursive,
    Fantasy,
    SystemUi,
    UiSerif,
    UiSansSerif,
    UiMonospace,
    UiRounded,
    Emoji,
    Math,
    Fangsong,
}

impl<'a> TryFrom<&'a str> for Generic {
    type Error = cssparser::ParseError<'a, ()>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut parse_input = cssparser::ParserInput::new(value);
        let mut parser = cssparser::Parser::new(&mut parse_input);
        let token = parser.next()?;
        if let cssparser::Token::Ident(s) = token {
            let first = match s.as_ref() {
                SERIF_STR => Ok(Generic::Serif),
                SANS_SERIF_STR => Ok(Generic::SansSerif),
                MONOSPACE_STR => Ok(Generic::Monospace),
                CURSIVE_STR => Ok(Generic::Cursive),
                FANTASY_STR => Ok(Generic::Fantasy),
                SYSTEM_UI_STR => Ok(Generic::SystemUi),
                UI_SERIF_STR => Ok(Generic::UiSerif),
                UI_SANS_SERIF_STR => Ok(Generic::UiSansSerif),
                UI_MONOSPACE_STR => Ok(Generic::UiMonospace),
                UI_ROUNDED_STR => Ok(Generic::UiRounded),
                EMOJI_STR => Ok(Generic::Emoji),
                MATH_STR => Ok(Generic::Math),
                FANGSONG_STR => Ok(Generic::Fangsong),
                _ => Err(cssparser::ParseError {
                    kind: cssparser::ParseErrorKind::Basic(
                        cssparser::BasicParseErrorKind::UnexpectedToken(
                            token.to_owned(),
                        ),
                    ),
                    location: parser.current_source_location(),
                }),
            }?;
            if parser.is_exhausted() {
                Ok(first)
            } else {
                Err(cssparser::ParseError {
                    kind: cssparser::ParseErrorKind::Basic(
                        cssparser::BasicParseErrorKind::UnexpectedToken(
                            parser.next()?.to_owned(),
                        ),
                    ),
                    location: parser.current_source_location(),
                })
            }
        } else {
            return Err(cssparser::ParseError {
                kind: cssparser::ParseErrorKind::Basic(
                    cssparser::BasicParseErrorKind::UnexpectedToken(
                        token.to_owned(),
                    ),
                ),
                location: parser.current_source_location(),
            });
        }
    }
}

impl cssparser::ToCss for Generic {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        dest.write_str(self.as_ref())
    }
}

impl std::fmt::Display for Generic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_css(f)
    }
}

impl AsRef<str> for Generic {
    fn as_ref(&self) -> &str {
        match self {
            Generic::Serif => SERIF_STR,
            Generic::SansSerif => SANS_SERIF_STR,
            Generic::Monospace => MONOSPACE_STR,
            Generic::Cursive => CURSIVE_STR,
            Generic::Fantasy => FANTASY_STR,
            Generic::SystemUi => SYSTEM_UI_STR,
            Generic::UiSerif => UI_SERIF_STR,
            Generic::UiSansSerif => UI_SANS_SERIF_STR,
            Generic::UiMonospace => UI_MONOSPACE_STR,
            Generic::UiRounded => UI_ROUNDED_STR,
            Generic::Emoji => EMOJI_STR,
            Generic::Math => MATH_STR,
            Generic::Fangsong => FANGSONG_STR,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serif() {
        let input = SERIF_STR;
        let font_family: Generic = input.try_into().unwrap();
        assert_eq!(input, font_family.as_ref());
        assert_eq!(font_family, Generic::Serif);
    }

    #[test]
    fn sans_serif() {
        let input = SANS_SERIF_STR;
        let font_family: Generic = input.try_into().unwrap();
        assert_eq!(input, font_family.as_ref());
        assert_eq!(font_family, Generic::SansSerif);
    }

    #[test]
    fn monospace() {
        let input = MONOSPACE_STR;
        let font_family: Generic = input.try_into().unwrap();
        assert_eq!(input, font_family.as_ref());
        assert_eq!(font_family, Generic::Monospace);
    }

    #[test]
    fn cursive() {
        let input = CURSIVE_STR;
        let font_family: Generic = input.try_into().unwrap();
        assert_eq!(input, font_family.as_ref());
        assert_eq!(font_family, Generic::Cursive);
    }

    #[test]
    fn fantasy() {
        let input = FANTASY_STR;
        let font_family: Generic = input.try_into().unwrap();
        assert_eq!(input, font_family.as_ref());
        assert_eq!(font_family, Generic::Fantasy);
    }

    #[test]
    fn system_ui() {
        let input = SYSTEM_UI_STR;
        let font_family: Generic = input.try_into().unwrap();
        assert_eq!(input, font_family.as_ref());
        assert_eq!(font_family, Generic::SystemUi);
    }

    #[test]
    fn ui_serif() {
        let input = UI_SERIF_STR;
        let font_family: Generic = input.try_into().unwrap();
        assert_eq!(input, font_family.as_ref());
        assert_eq!(font_family, Generic::UiSerif);
    }

    #[test]
    fn ui_sans_serif() {
        let input = UI_SANS_SERIF_STR;
        let font_family: Generic = input.try_into().unwrap();
        assert_eq!(input, font_family.as_ref());
        assert_eq!(font_family, Generic::UiSansSerif);
    }

    #[test]
    fn ui_monospace() {
        let input = UI_MONOSPACE_STR;
        let font_family: Generic = input.try_into().unwrap();
        assert_eq!(input, font_family.as_ref());
        assert_eq!(font_family, Generic::UiMonospace);
    }

    #[test]
    fn ui_rounded() {
        let input = UI_ROUNDED_STR;
        let font_family: Generic = input.try_into().unwrap();
        assert_eq!(input, font_family.as_ref());
        assert_eq!(font_family, Generic::UiRounded);
    }

    #[test]
    fn emoji() {
        let input = EMOJI_STR;
        let font_family: Generic = input.try_into().unwrap();
        assert_eq!(input, font_family.as_ref());
        assert_eq!(font_family, Generic::Emoji);
    }

    #[test]
    fn math() {
        let input = MATH_STR;
        let font_family: Generic = input.try_into().unwrap();
        assert_eq!(input, font_family.as_ref());
        assert_eq!(font_family, Generic::Math);
    }

    #[test]
    fn fangsong() {
        let input = FANGSONG_STR;
        let font_family: Generic = input.try_into().unwrap();
        assert_eq!(input, font_family.as_ref());
        assert_eq!(font_family, Generic::Fangsong);
    }

    #[test]
    fn invalid() {
        let input = "invalid";
        let font_family: Result<Generic, _> = input.try_into();
        assert!(font_family.is_err());
    }

    #[test]
    fn empty() {
        let input = "";
        let font_family: Result<Generic, _> = input.try_into();
        assert!(font_family.is_err());
    }

    #[test]
    fn unexhausted() {
        let input = format!("{SERIF_STR} invalid");
        let font_family: Result<Generic, _> = input.as_str().try_into();
        assert!(font_family.is_err());
    }
}
