use cssparser::ToCss as _;

/// Owned version of `cssparser::Token::Dimension`.
///
/// https://docs.rs/cssparser/latest/cssparser/enum.Token.html#variant.Dimension
/// https://drafts.csswg.org/css-syntax/#dimension-token-diagram
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Dimension {
    has_sign: bool,
    value: f32,
    int_value: Option<i32>,
    unit: String,
}

impl<'a> TryFrom<&'a str> for Dimension {
    type Error = cssparser::BasicParseError<'a>;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut parser_input = cssparser::ParserInput::new(value);
        let mut parser = cssparser::Parser::new(&mut parser_input);
        let token = parser.next()?;
        if let cssparser::Token::Dimension {
            has_sign,
            value,
            int_value,
            unit,
        } = token
        {
            let dimension = Self {
                has_sign: *has_sign,
                value: *value,
                int_value: *int_value,
                unit: unit.to_string(),
            };
            if parser.is_exhausted() {
                Ok(dimension)
            } else {
                let e = Self::Error {
                    kind: cssparser::BasicParseErrorKind::UnexpectedToken(
                        parser.next()?.to_owned(),
                    ),
                    location: parser.current_source_location(),
                };
                Err(e)
            }
        } else {
            let e = Self::Error {
                kind: cssparser::BasicParseErrorKind::UnexpectedToken(
                    token.to_owned(),
                ),
                location: parser.current_source_location(),
            };
            Err(e)
        }
    }
}

impl cssparser::ToCss for Dimension {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        cssparser::Token::Dimension {
            has_sign: self.has_sign,
            value: self.value,
            int_value: self.int_value,
            unit: self.unit.as_str().into(),
        }
        .to_css(dest)
    }
}

impl std::fmt::Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_css(f)
    }
}

#[cfg(test)]
mod tests {
    use super::Dimension;

    #[test]
    fn negative_rem() {
        let input = "-1rem";
        let dimension = Dimension::try_from(input).unwrap();
        assert!(dimension.has_sign);
        assert_eq!(dimension.value, -1.0);
        assert_eq!(dimension.int_value, Some(-1));
        assert_eq!(dimension.unit, "rem");
        assert_eq!(input, format!("{}", dimension));
    }

    #[test]
    fn px_frac() {
        let input = "30.5px";
        let dimension = Dimension::try_from(input).unwrap();
        assert!(!dimension.has_sign);
        assert_eq!(dimension.value, 30.5);
        assert_eq!(dimension.int_value, None);
        assert_eq!(dimension.unit, "px");
        assert_eq!(input, format!("{}", dimension));
    }

    #[test]
    fn invalid() {
        let input = "invalid";
        let dimension = Dimension::try_from(input);
        assert!(dimension.is_err());
    }

    #[test]
    fn empty() {
        let input = "";
        let dimension = Dimension::try_from(input);
        assert!(dimension.is_err());
    }

    #[test]
    fn unexhausted() {
        let input = "30px invalid";
        let dimension = Dimension::try_from(input);
        assert!(dimension.is_err());
    }
}
