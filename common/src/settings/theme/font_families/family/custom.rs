use cssparser::ToCss as _;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Custom(String);

impl<'a> TryFrom<&'a str> for Custom {
    type Error = cssparser::ParseError<'a, ()>;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        // Prepare parser.
        let mut parse_input = cssparser::ParserInput::new(value);
        let mut parser = cssparser::Parser::new(&mut parse_input);
        // Get token.
        let token = parser.try_next()?;
        if let cssparser::Token::QuotedString(s) = token {
            // Try to parse token as `QuotedString`.
            let string = format!("\"{}\"", s);
            if parser.is_exhausted() {
                // Is a `Custom`.
                Ok(Custom(string))
            } else {
                // Too many tokens.
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
            // Not a `QuotedString`.
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

impl cssparser::ToCss for Custom {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        dest.write_str(&self.0)
    }
}

impl std::fmt::Display for Custom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_css(f)
    }
}

impl AsRef<str> for Custom {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom() {
        let input = "\"Lato\"";
        let custom: Custom = input.try_into().unwrap();
        assert_eq!(input, custom.as_ref());
        assert_eq!(input, Custom("\"Lato\"".into()).as_ref());
    }

    #[test]
    fn invalid() {
        let input = "invalid";
        let result: Result<Custom, _> = input.try_into();
        assert!(result.is_err());
    }

    #[test]
    fn empty() {
        let input = "";
        let result: Result<Custom, _> = input.try_into();
        assert!(result.is_err());
    }

    #[test]
    fn unexhausted() {
        let input = "\"Lato\" invalid";
        let result: Result<Custom, _> = input.try_into();
        assert!(result.is_err());
    }
}
