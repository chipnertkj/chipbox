use super::global::Global;
use cssparser::ToCss as _;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Color {
    Color(cssparser_color::Color),
    Global(Global),
}

impl<'a> TryFrom<&'a str> for Color {
    type Error = cssparser::ParseError<'a, ()>;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut parser_input = cssparser::ParserInput::new(value);
        let mut parser = cssparser::Parser::new(&mut parser_input);
        // Try to parse as `Color`.
        let color_result = cssparser_color::Color::parse(&mut parser);
        if let Ok(color) = color_result {
            // Is a `Color`.
            if parser.is_exhausted() {
                // Parser exhausted.
                Ok(Color::Color(color))
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
            // Parse as `Global`.
            let global = Global::try_from(value)?;
            if parser.is_exhausted() {
                // Parser exhausted.
                Ok(Color::Global(global))
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
        }
    }
}

impl cssparser::ToCss for Color {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        match self {
            Color::Color(color) => color.to_css(dest),
            Color::Global(global) => global.to_css(dest),
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_css(f)
    }
}

#[cfg(test)]
mod tests {
    use cssparser::ToCss;

    use crate::settings::theme::global::Global;

    use super::Color;

    #[test]
    fn rgba() {
        let input = "rgba(0, 0, 0, 0)";
        let color = Color::try_from(input).unwrap();
        assert_eq!(input, format!("{}", color));
        assert_eq!(
            color,
            Color::Color(cssparser_color::Color::Rgba(
                cssparser_color::RgbaLegacy {
                    red: 0,
                    green: 0,
                    blue: 0,
                    alpha: 0.,
                }
            ))
        );
    }

    #[test]
    fn rgb() {
        let input = "rgb(0, 0, 0)";
        let color = Color::try_from(input).unwrap();
        let expected = Color::Color(cssparser_color::Color::Rgba(
            cssparser_color::RgbaLegacy {
                red: 0,
                green: 0,
                blue: 0,
                alpha: 1.,
            },
        ));
        assert_eq!(input, format!("{}", color));
        assert_eq!(color, expected);
    }

    #[test]
    fn hex_8() {
        let input = "#00000000";
        let color = Color::try_from(input).unwrap();
        let expected = Color::Color(cssparser_color::Color::Rgba(
            cssparser_color::RgbaLegacy {
                red: 0,
                green: 0,
                blue: 0,
                alpha: 0.,
            },
        ));
        assert_eq!(expected.to_css_string(), format!("{}", color));
        assert_eq!(color, expected);
    }

    #[test]
    fn hex_6() {
        let input = "#000000";
        let color = Color::try_from(input).unwrap();
        let expected = Color::Color(cssparser_color::Color::Rgba(
            cssparser_color::RgbaLegacy {
                red: 0,
                green: 0,
                blue: 0,
                alpha: 1.,
            },
        ));
        assert_eq!(expected.to_css_string(), format!("{}", color));
        assert_eq!(color, expected);
    }

    #[test]
    fn hex_3() {
        let input = "#000";
        let color = Color::try_from(input).unwrap();
        let expected = Color::Color(cssparser_color::Color::Rgba(
            cssparser_color::RgbaLegacy {
                red: 0,
                green: 0,
                blue: 0,
                alpha: 1.,
            },
        ));
        assert_eq!(expected.to_css_string(), format!("{}", color));
        assert_eq!(color, expected);
    }

    #[test]
    fn hex_4() {
        let input = "#0000";
        let color = Color::try_from(input).unwrap();
        let expected = Color::Color(cssparser_color::Color::Rgba(
            cssparser_color::RgbaLegacy {
                red: 0,
                green: 0,
                blue: 0,
                alpha: 0.,
            },
        ));
        assert_eq!(expected.to_css_string(), format!("{}", color));
        assert_eq!(color, expected);
    }

    #[test]
    fn hsl() {
        let input = "hsl(0 0% 0%)";
        let color = Color::try_from(input).unwrap();
        let expected =
            Color::Color(cssparser_color::Color::Hsl(cssparser_color::Hsl {
                hue: Some(0.),
                saturation: Some(0.),
                lightness: Some(0.),
                alpha: Some(1.),
            }));
        assert_eq!(expected.to_css_string(), format!("{}", color));
        assert_eq!(color, expected);
    }

    #[test]
    fn hsla() {
        let input = "hsla(0, 0%, 0%, 0)";
        let color = Color::try_from(input).unwrap();
        let expected =
            Color::Color(cssparser_color::Color::Hsl(cssparser_color::Hsl {
                hue: Some(0.),
                saturation: Some(0.),
                lightness: Some(0.),
                alpha: Some(0.),
            }));
        assert_eq!(expected.to_css_string(), format!("{}", color));
        assert_eq!(color, expected);
    }

    #[test]
    fn global() {
        let input = "inherit";
        let color = Color::try_from(input).unwrap();
        let expected = Color::Global(Global::Inherit);
        assert_eq!(expected.to_css_string(), format!("{}", color));
        assert_eq!(color, expected);
    }

    #[test]
    fn invalid() {
        let input = "invalid";
        let color = Color::try_from(input);
        assert!(color.is_err());
    }

    #[test]
    fn empty() {
        let input = "";
        let color = Color::try_from(input);
        assert!(color.is_err());
    }

    #[test]
    fn unexhausted() {
        let input = "#000 invalid";
        let color = Color::try_from(input);
        assert!(color.is_err());
    }
}
