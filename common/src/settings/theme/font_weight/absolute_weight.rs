use cssparser::ToCss as _;

pub const NORMAL_STR: &str = "normal";
pub const BOLD_STR: &str = "bold";

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum AbsoluteWeight {
    Normal,
    Bold,
    Numeric(f32),
}

impl<'a> TryFrom<&'a str> for AbsoluteWeight {
    type Error = cssparser::ParseError<'a, ()>;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        // Prepare parser.
        let mut parser_input = cssparser::ParserInput::new(value);
        let mut parser = cssparser::Parser::new(&mut parser_input);
        // Try to parse as `Ident`.
        let token = parser.try_next()?;
        if let cssparser::Token::Ident(ident) = token {
            // Is an `Ident`.
            let weight = match ident.as_ref() {
                NORMAL_STR => Ok(AbsoluteWeight::Normal),
                BOLD_STR => Ok(AbsoluteWeight::Bold),
                // Invalid `Ident`.
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
                // Parser exhausted.
                Ok(weight)
            } else {
                // Parser not exhausted.
                Err(cssparser::ParseError {
                    kind: cssparser::ParseErrorKind::Basic(
                        cssparser::BasicParseErrorKind::UnexpectedToken(
                            parser.try_next()?.to_owned(),
                        ),
                    ),
                    location: parser.current_source_location(),
                })
            }
        } else {
            // Parse as `Number`.
            if let cssparser::Token::Number { value, .. } = token {
                // Is a `Number`.
                // https://developer.mozilla.org/en-US/docs/Web/CSS/font-weight#number
                let weight =
                    Ok(AbsoluteWeight::Numeric(value.clamp(0., 1000.)));
                if parser.is_exhausted() {
                    // Parser exhausted.
                    weight
                } else {
                    // Parser not exhausted.
                    Err(cssparser::ParseError {
                        kind: cssparser::ParseErrorKind::Basic(
                            cssparser::BasicParseErrorKind::UnexpectedToken(
                                parser.try_next()?.to_owned(),
                            ),
                        ),
                        location: parser.current_source_location(),
                    })
                }
            } else {
                // Not a `Number` or valid `Ident`.
                Err(cssparser::ParseError {
                    kind: cssparser::ParseErrorKind::Basic(
                        cssparser::BasicParseErrorKind::UnexpectedToken(
                            token.to_owned(),
                        ),
                    ),
                    location: parser.current_source_location(),
                })
            }
        }
    }
}

impl cssparser::ToCss for AbsoluteWeight {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        match self {
            AbsoluteWeight::Normal => dest.write_str(NORMAL_STR),
            AbsoluteWeight::Bold => dest.write_str(BOLD_STR),
            AbsoluteWeight::Numeric(value) => value.to_css(dest),
        }
    }
}

impl std::fmt::Display for AbsoluteWeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_css(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal() {
        assert_eq!(AbsoluteWeight::Normal, NORMAL_STR.try_into().unwrap());
    }

    #[test]
    fn bold() {
        assert_eq!(AbsoluteWeight::Bold, BOLD_STR.try_into().unwrap());
    }

    #[test]
    fn numeric() {
        assert_eq!(AbsoluteWeight::Numeric(100.), "100".try_into().unwrap());
    }

    #[test]
    fn invalid() {
        assert!(AbsoluteWeight::try_from("invalid").is_err());
    }

    #[test]
    fn empty() {
        assert!(AbsoluteWeight::try_from("").is_err());
    }

    #[test]
    fn clamp() {
        assert_eq!(AbsoluteWeight::Numeric(1000.), "1500".try_into().unwrap());
        assert_eq!(AbsoluteWeight::Numeric(0.), "-1500".try_into().unwrap());
    }

    #[test]
    fn unexhausted() {
        assert!(AbsoluteWeight::try_from(
            format!("{BOLD_STR} invalid").as_str()
        )
        .is_err());
    }
}
