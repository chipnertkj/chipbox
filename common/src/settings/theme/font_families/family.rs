use cssparser::ToCss as _;
mod generic;

pub use crate::settings::theme::global::Global;
pub use generic::{
    Generic, CURSIVE_STR, EMOJI_STR, FANGSONG_STR, FANTASY_STR, MATH_STR,
    MONOSPACE_STR, SANS_SERIF_STR, SERIF_STR, SYSTEM_UI_STR, UI_MONOSPACE_STR,
    UI_ROUNDED_STR, UI_SANS_SERIF_STR, UI_SERIF_STR,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum FontFamily {
    GlobalValue(Global),
    Generic(Generic),
    Custom(String),
}

impl<'a> TryFrom<&'a str> for FontFamily {
    type Error = cssparser::ParseError<'a, ()>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        // Prepare parser.
        let mut parse_input = cssparser::ParserInput::new(value);
        let mut parser = cssparser::Parser::new(&mut parse_input);
        // Get token.
        let token = parser.try_next()?;
        // Convert to string for further parsing.
        let token_string = token.to_css_string();
        // Try to parse token as `Global`.
        let global_result = token_string
            .as_str()
            .try_into();
        if let Ok(global) = global_result {
            // Is a `Global`.
            Ok(FontFamily::GlobalValue(global))
        } else {
            // Try to parse token as `Generic`.
            let generic_result = token_string
                .as_str()
                .try_into();
            if let Ok(generic) = generic_result {
                // Is a `Generic`.
                Ok(FontFamily::Generic(generic))
            } else if let cssparser::Token::QuotedString(s) = token {
                // Try to parse token as `Custom`.
                let string = s.to_string();
                if parser.is_exhausted() {
                    // Is a `Custom`.
                    Ok(FontFamily::Custom(string))
                } else {
                    // Is something else. Unexpected token.
                    Err(cssparser::ParseError {
                        kind: cssparser::ParseErrorKind::Basic(
                            cssparser::BasicParseErrorKind::UnexpectedToken(
                                parser.try_next()?.clone(),
                            ),
                        ),
                        location: parser.current_source_location(),
                    })
                }
            } else {
                // Should contain only one token.
                Err(cssparser::ParseError {
                    kind: cssparser::ParseErrorKind::Basic(
                        cssparser::BasicParseErrorKind::UnexpectedToken(
                            token.clone(),
                        ),
                    ),
                    location: parser.current_source_location(),
                })
            }
        }
    }
}

impl cssparser::ToCss for FontFamily {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        match self {
            FontFamily::GlobalValue(global) => global.to_css(dest),
            FontFamily::Generic(generic) => generic.to_css(dest),
            FontFamily::Custom(custom) => dest.write_str(custom),
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
            FontFamily::GlobalValue(global) => global.as_ref(),
            FontFamily::Generic(generic) => generic.as_ref(),
            FontFamily::Custom(custom) => custom,
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
        assert_eq!(font_family, FontFamily::Custom("Lato".into()));
    }

    #[test]
    fn family_global() {
        let input = "inherit";
        let font_family: FontFamily = input.try_into().unwrap();
        assert_eq!(input, font_family.as_ref());
        assert_eq!(font_family, FontFamily::GlobalValue(Global::Inherit));
    }
}
