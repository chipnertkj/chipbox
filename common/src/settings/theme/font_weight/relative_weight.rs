use cssparser::ToCss as _;

pub const LIGHTER_STR: &str = "lighter";
pub const BOLDER_STR: &str = "bolder";

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
/// https://developer.mozilla.org/en-US/docs/Web/CSS/font-weight
pub enum RelativeWeight {
    Lighter,
    Bolder,
}

impl<'a> TryFrom<&'a str> for RelativeWeight {
    type Error = cssparser::ParseError<'a, ()>;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut parser_input = cssparser::ParserInput::new(value);
        let mut parser = cssparser::Parser::new(&mut parser_input);
        let token = parser.next()?;
        if let cssparser::Token::Ident(ident) = token {
            let weight = match ident.as_ref() {
                LIGHTER_STR => Ok(RelativeWeight::Lighter),
                BOLDER_STR => Ok(RelativeWeight::Bolder),
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
                            parser.next()?.to_owned(),
                        ),
                    ),
                    location: parser.current_source_location(),
                })
            }
        } else {
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

impl cssparser::ToCss for RelativeWeight {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        dest.write_str(self.as_ref())
    }
}

impl std::fmt::Display for RelativeWeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_css(f)
    }
}

impl AsRef<str> for RelativeWeight {
    fn as_ref(&self) -> &str {
        match self {
            RelativeWeight::Lighter => LIGHTER_STR,
            RelativeWeight::Bolder => BOLDER_STR,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lighter() {
        let weight = RelativeWeight::try_from(LIGHTER_STR).unwrap();
        assert_eq!(weight, RelativeWeight::Lighter);
    }

    #[test]
    fn bolder() {
        let weight = RelativeWeight::try_from(BOLDER_STR).unwrap();
        assert_eq!(weight, RelativeWeight::Bolder);
    }

    #[test]
    fn invalid() {
        let weight = RelativeWeight::try_from("invalid");
        assert!(weight.is_err());
    }

    #[test]
    fn empty() {
        let weight = RelativeWeight::try_from("");
        assert!(weight.is_err());
    }

    #[test]
    fn unexhausted() {
        let input = format!("{BOLDER_STR} invalid");
        let weight = RelativeWeight::try_from(input.as_str());
        assert!(weight.is_err());
    }
}
