use crate::settings::theme::*;

pub fn theme() -> Theme {
    Theme {
        name: name(),
        fonts: fonts(),
        text: text(),
    }
}

fn name() -> String {
    "chipbox-dark".into()
}

fn fonts() -> FontTheme {
    FontTheme {
        family_sans: "\"Lato\", sans-serif"
            .try_into()
            .unwrap(),
        family_mono: "\"IBM Plex Mono\", monospace"
            .try_into()
            .unwrap(),
    }
}

fn text() -> TextTheme {
    TextTheme {
        primary: text_primary(),
        secondary: text_secondary(),
        tertiary: text_tertiary(),
    }
}

fn text_primary() -> TextProps {
    TextProps {
        color: "hsl(0, 0%, 90%)"
            .try_into()
            .unwrap(),
        font_size: "1rem".try_into().unwrap(),
        font_weight: "400".try_into().unwrap(),
        line_height: "1.15rem".try_into().unwrap(),
    }
}

fn text_secondary() -> TextProps {
    TextProps {
        color: "hsl(0, 0%, 60%)"
            .try_into()
            .unwrap(),
        font_size: "1rem".try_into().unwrap(),
        font_weight: "400".try_into().unwrap(),
        line_height: "1.15rem".try_into().unwrap(),
    }
}

fn text_tertiary() -> TextProps {
    TextProps {
        color: "hsl(0, 0%, 40%)"
            .try_into()
            .unwrap(),
        font_size: "1rem".try_into().unwrap(),
        font_weight: "400".try_into().unwrap(),
        line_height: "1.15rem".try_into().unwrap(),
    }
}
