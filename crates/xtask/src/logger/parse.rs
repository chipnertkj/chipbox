use std::mem::MaybeUninit;

use ratatui::{
    style::{Color, Modifier, Style},
    text::Line,
};
use ringbuf::traits::Consumer as _;

use self::performer::LogVtePerformer;
use crate::logger::{LogData, LogKind, Logger};

mod performer;
mod sgr;

// Allocate buffer for peeking entries
fn allocate_uninit_buffer(buf_len: usize) -> Vec<MaybeUninit<LogData>> {
    let mut uninit_entries = Vec::with_capacity(buf_len);
    // SAFETY: `set_len` is safe because of `with_capacity` invariant.
    unsafe { uninit_entries.set_len(buf_len) };
    uninit_entries
}

/// Iterates over initialized entries in the logger.
///
/// ## Safety
/// All [`MaybeUninit`] entries must be initialized before calling this function.
unsafe fn entries_with_partial<'a>(
    initialized_entries: &'a [MaybeUninit<LogData>],
    logger: &'a Logger,
) -> Box<dyn Iterator<Item = (LogKind, &'a [u8])> + 'a> {
    // Process initialized entries into data references.
    let entries = || {
        initialized_entries.iter().map(|entry| {
            // SAFETY: assume_init_ref is safe because iter must be safe
            let entry = unsafe { entry.assume_init_ref() };
            (entry.kind, entry.buf.as_slice())
        })
    };
    // If the partial buffer is empty, return the iterator.
    // Otherwise, return the iterator plus the partial buffer.
    logger.partial.contents().map_or_else(
        || Box::new(entries()) as Box<dyn Iterator<Item = _>>,
        |partial| {
            let chained = entries().chain(std::iter::once(partial));
            Box::new(chained)
        },
    )
}

/// Converts entries in the logger to a vector of [`ratatui::text::Line`]s.
pub fn to_lines(logger: &Logger) -> Vec<Line<'_>> {
    let mut uninit_entries = allocate_uninit_buffer(logger.len());
    let valid_entries = logger.entries.peek_slice_uninit(&mut uninit_entries);
    let mut parser = vte::Parser::new();
    let mut performer = LogVtePerformer::with_capacity(logger.len());
    let init_entries = &uninit_entries[..valid_entries];
    // SAFETY: peek_slice_uninit initialized the entries until num_entries
    let entries = unsafe { entries_with_partial(init_entries, logger) };
    // Process entries.
    let mut last_kind = None;
    entries.for_each(|(kind, entry_buf)| {
        // Apply new default style if the kind has changed.
        let apply_style = last_kind != Some(kind);
        // Assign base style depending on log kind.
        if apply_style {
            performer.set_default_style(log_style(kind));
        }
        parser.advance(&mut performer, entry_buf);
        performer.flush_line_partial();
        // Update persistent state.
        last_kind = Some(kind);
    });
    performer.into_output()
}

/// Returns the base style for a given log kind.
fn log_style(kind: LogKind) -> Style {
    match kind {
        LogKind::Stdout => Style::default(),
        LogKind::Stderr => Style::default().add_modifier(Modifier::ITALIC),
        LogKind::XTask => Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::ITALIC),
    }
}
