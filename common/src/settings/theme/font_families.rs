use cssparser::ToCss as _;
mod family;

#[allow(unused_imports)]
pub use family::{
    FontFamily, Generic, CURSIVE_STR, EMOJI_STR, FANGSONG_STR, FANTASY_STR,
    MATH_STR, MONOSPACE_STR, SANS_SERIF_STR, SERIF_STR, SYSTEM_UI_STR,
    UI_MONOSPACE_STR, UI_ROUNDED_STR, UI_SANS_SERIF_STR, UI_SERIF_STR,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct FontFamilies(Vec<FontFamily>);

impl<'a> TryFrom<&'a str> for FontFamilies {
    type Error = cssparser::ParseError<'a, ()>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let families = value
            // Split by comma and trim.
            .split(',')
            .map(|s| s.trim())
            // Remove empty items.
            .filter(|s| !s.is_empty())
            // Convert to `FontFamily`.
            .map(|s| s.try_into())
            // Collect into `Vec`.
            .collect::<Result<_, _>>()?;
        Ok(FontFamilies(families))
    }
}

impl cssparser::ToCss for FontFamilies {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        let mut iter = self.0.iter();
        if let Some(first) = iter.next() {
            dest.write_str(first.as_ref())?;
            for item in iter {
                dest.write_str(", ")?;
                dest.write_str(item.as_ref())?;
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for FontFamilies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_css(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn families() {
        let input = format!(
            "{sans_serif}, {serif}, \"Lato\"",
            sans_serif = family::SANS_SERIF_STR,
            serif = family::SERIF_STR
        );
        let font_families: FontFamilies = input
            .as_str()
            .try_into()
            .unwrap();
        let expected = FontFamilies(vec![
            FontFamily::Generic(family::Generic::SansSerif),
            FontFamily::Generic(family::Generic::Serif),
            "\"Lato\"".try_into().unwrap(),
        ]);
        assert_eq!(font_families, expected);
    }
}
