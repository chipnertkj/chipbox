use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

use crate::logger::parse::sgr;

#[derive(Debug)]
pub struct LogVtePerformer<'text> {
    // Final output.
    output: Vec<Line<'text>>,
    // Current style, applied to the next span.
    style: Style,
    // Style used for style reset.
    default_style: Style,
    // Current line being built.
    line_partial: Vec<Span<'text>>,
    // Buffer for current span.
    span_partial: String,
}

impl<'text> LogVtePerformer<'text> {
    /// Initializes performer state.
    /// `lines` is the expected number of lines to allocate.
    pub fn with_capacity(lines: usize) -> Self {
        Self {
            output: Vec::with_capacity(lines),
            style: Style::default(),
            default_style: Style::default(),
            line_partial: Vec::new(),
            span_partial: String::new(),
        }
    }

    /// Sets the default style used for style reset operations.
    /// Also apply it as the current style.
    pub const fn set_default_style(&mut self, style: Style) {
        self.default_style = style;
        self.style = style;
    }

    /// Creates a new span baased on span partial and adds it to the line partial.
    /// This also clears the current span partial.
    fn flush_span_partial(&mut self) {
        if !self.span_partial.is_empty() {
            let span = Span {
                style: self.style,
                content: self.span_partial.clone().into(),
            };
            // Could annotate span with hyperlink metadata here,
            // but `ratatui` doesn't support it yet.
            self.line_partial.push(span);
            self.span_partial.clear();
        }
    }

    /// Finishes the line partial and adds it to the output.
    /// This also clears the line partial.
    pub fn flush_line_partial(&mut self) {
        self.flush_span_partial();
        if !self.line_partial.is_empty() {
            self.output.push(Line {
                style: Style::default(),
                alignment: None,
                spans: std::mem::take(&mut self.line_partial),
            });
        }
    }

    /// Appends text to the span partial.
    fn append_span_text(&mut self, text: &str) {
        self.span_partial.push_str(text);
    }

    /// Consumes the performer and returns the output.
    pub fn into_output(self) -> Vec<Line<'text>> {
        self.output
    }
}

impl vte::Perform for LogVtePerformer<'_> {
    fn print(&mut self, c: char) {
        self.append_span_text(&c.to_string());
    }

    fn execute(&mut self, byte: u8) {
        if byte == b'\n' {
            self.flush_line_partial();
        }
    }

    fn csi_dispatch(
        &mut self,
        params: &vte::Params,
        _intermediates: &[u8],
        _ignore: bool,
        action: char,
    ) {
        let expect_indexed =
            |index: u16| Color::Indexed(index.try_into().expect("index param was not one byte"));
        let expect_rgb = |r: u16, g: u16, b: u16| {
            let [r, g, b]: [u8; 3] = [r, g, b]
                .map(TryInto::try_into)
                .map(|r| r.expect("rgb param was not one byte"));
            Color::Rgb(r, g, b)
        };
        if action == sgr::ACTION {
            self.flush_span_partial(); // style change = flush current span
            let flat: Vec<_> = params.iter().flatten().copied().collect();

            let style = match *flat.as_slice() {
                [sgr::RESET] => Style::default(),
                [sgr::BOLD] => self.style.add_modifier(Modifier::BOLD),
                [sgr::ITALIC] => self.style.add_modifier(Modifier::ITALIC),
                [sgr::UNDERLINE] => self.style.add_modifier(Modifier::UNDERLINED),
                [param @ sgr::FG_FIRST..=sgr::FG_LAST] => self
                    .style
                    .fg(sgr::fg_color(param).expect("matched within range")),
                [param @ sgr::BG_FIRST..=sgr::BG_LAST] => self
                    .style
                    .bg(sgr::bg_color(param).expect("matched within range")),
                [sgr::FG_RGB, sgr::RGB_DIRECT, r, g, b] => self.style.fg(expect_rgb(r, g, b)),
                [sgr::BG_RGB, sgr::RGB_DIRECT, r, g, b] => self.style.bg(expect_rgb(r, g, b)),
                [sgr::FG_RGB, sgr::RGB_256_PALETTE, index] => self.style.fg(expect_indexed(index)),
                [sgr::BG_RGB, sgr::RGB_256_PALETTE, index] => self.style.bg(expect_indexed(index)),
                _ => self.style,
            };
            self.style = style;
        }
    }

    fn osc_dispatch(&mut self, params: &[&[u8]], _bell_terminated: bool) {
        if params.first().is_some_and(|v| *v == b"8") {
            // The commented out implementation here preserves the sequence.
            // Currently, this is not supported by `ratatui`.
            // // Reconstruct the original OSC8 sequence exactly.
            // let mut seq = String::from("\x1b]8");
            // // Iterate through all parameters, adding each with a semicolon.
            // params
            //     .iter()
            //     // Skip the initial 8.
            //     .skip(1)
            //     .for_each(|param| {
            //         seq.push(';');
            //         seq.push_str(&String::from_utf8_lossy(param));
            //     });
            // if bell_terminated {
            //     seq.push('\x07'); // BEL termination
            // } else {
            //     seq.push_str("\x1b\\"); // ESC backslash termination
            // }
            // self.append_span_text(&seq);
            let mut seq = String::new();
            // Skip the initial 8, OSC8 params and URL.
            if let Some(v) = params.get(3) {
                seq.push_str(&String::from_utf8_lossy(v));
            }
            self.append_span_text(&seq);
        }
    }
}
