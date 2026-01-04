//! Partial UTF-8 chunk handling.

use crate::logger::{LogData, LogKind};

/// A partial buffer for capturing a potentially
/// incomplete chunk of UTF-8 data.
#[derive(Debug)]
pub struct LogPartial {
    buf: Vec<u8>,
    /// The last kind of data that was added to the partial buffer.
    /// `None` if the buffer has never been written to.
    last_kind: Option<LogKind>,
    /// The maximum length of the partial buffer.
    capacity: usize,
}

impl LogPartial {
    /// Creates a new partial buffer with the given maximum length.
    pub fn new(capacity: usize) -> Self {
        Self {
            last_kind: None,
            buf: Vec::with_capacity(capacity),
            capacity,
        }
    }

    /// Returns whether the partial buffer is full.
    pub const fn is_full(&self) -> bool {
        self.buf.len() == self.capacity
    }

    /// Returns whether the partial buffer is empty.
    pub const fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    /// Takes the partial buffer and converts it into a [`LogEntry`].
    ///
    /// This is equivalent to assuming the partial is complete.
    pub fn flush(&mut self) -> Option<LogData> {
        self.last_kind.map(|kind| {
            let buf = self.buf.clone();
            self.buf.clear();
            LogData {
                kind,
                buf: buf.into(),
            }
        })
    }

    /// Returns the contents of the partial buffer.
    pub fn contents(&self) -> Option<(LogKind, &[u8])> {
        self.last_kind.map(|kind| (kind, self.buf.as_slice()))
    }

    /// Clears the partial buffer.
    pub fn clear(&mut self) {
        self.buf.clear();
        self.last_kind = None;
    }

    /// Extends the partial buffer with the given data.
    ///
    /// Returns the leftover data that did not fit into the partial buffer.
    ///
    /// If the `kind` value changes since last write, or the buffer is full,
    /// all of the data is returned as leftover.
    pub fn extend<'data>(&mut self, data: &'data [u8], kind: LogKind) -> &'data [u8] {
        // Cache state for a check ahead.
        let last_kind = self.last_kind;
        // Update state before returning control flow.
        self.last_kind = Some(kind);
        // Kind changed or cannot fit anything.
        if last_kind.is_some_and(|last_kind| last_kind != kind) || self.is_full() {
            // Return all as leftover.
            return data;
        }
        // Take the maximum amount of data that fits into the partial buffer.
        let take_n = (self.capacity - self.buf.len()).min(data.len());
        let (extend, leftover) = data.split_at(take_n);
        self.buf.extend_from_slice(extend);
        // Return the leftover data.
        leftover
    }
}
