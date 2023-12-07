use cssparser::ToCss as _;

const INHERIT_STR: &str = "inherit";
const INITIAL_STR: &str = "initial";
const REVERT_STR: &str = "revert";
const REVERT_LAYER_STR: &str = "revert-layer";
const UNSET_STR: &str = "unset";

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Global {
    Inherit,
    Initial,
    Revert,
    RevertLayer,
    Unset,
}

impl<'a> TryFrom<&'a str> for Global {
    type Error = cssparser::ParseError<'a, ()>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut parse_input = cssparser::ParserInput::new(value);
        let mut parser = cssparser::Parser::new(&mut parse_input);
        let token = parser.try_next()?;
        if let cssparser::Token::Ident(s) = token {
            let first = match s.as_ref() {
                INHERIT_STR => Ok(Self::Inherit),
                INITIAL_STR => Ok(Self::Initial),
                REVERT_STR => Ok(Self::Revert),
                REVERT_LAYER_STR => Ok(Self::RevertLayer),
                UNSET_STR => Ok(Self::Unset),
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
                            parser.try_next()?.to_owned(),
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

impl cssparser::ToCss for Global {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        dest.write_str(self.as_ref())
    }
}

impl std::fmt::Display for Global {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_css(f)
    }
}

impl AsRef<str> for Global {
    fn as_ref(&self) -> &str {
        match self {
            Global::Inherit => INHERIT_STR,
            Global::Initial => INITIAL_STR,
            Global::Revert => REVERT_STR,
            Global::RevertLayer => REVERT_LAYER_STR,
            Global::Unset => UNSET_STR,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inherit() {
        let input = INHERIT_STR;
        let global: Global = input.try_into().unwrap();
        assert_eq!(input, global.as_ref());
        assert_eq!(global, Global::Inherit);
    }

    #[test]
    fn initial() {
        let input = INITIAL_STR;
        let global: Global = input.try_into().unwrap();
        assert_eq!(input, global.as_ref());
        assert_eq!(global, Global::Initial);
    }

    #[test]
    fn revert() {
        let input = REVERT_STR;
        let global: Global = input.try_into().unwrap();
        assert_eq!(input, global.as_ref());
        assert_eq!(global, Global::Revert);
    }

    #[test]
    fn revert_layer() {
        let input = REVERT_LAYER_STR;
        let global: Global = input.try_into().unwrap();
        assert_eq!(input, global.as_ref());
        assert_eq!(global, Global::RevertLayer);
    }

    #[test]
    fn unset() {
        let input = UNSET_STR;
        let global: Global = input.try_into().unwrap();
        assert_eq!(input, global.as_ref());
        assert_eq!(global, Global::Unset);
    }

    #[test]
    fn invalid() {
        let input = "invalid";
        let global: Result<Global, _> = input.try_into();
        assert!(global.is_err());
    }
}
