use std::sync::Arc;

use ringbuf::{
    LocalRb,
    storage::Heap,
    traits::{Consumer as _, Observer as _, RingBuffer as _},
};

use self::partial::LogPartial;

mod parse;
mod partial;

/// The kind of data in a [`LogEntry`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LogKind {
    Stdout,
    Stderr,
    XTask,
}

/// A [`Logger`] entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogData {
    pub kind: LogKind,
    pub buf: Arc<Vec<u8>>,
}

impl LogData {
    pub fn new(kind: LogKind, buf: impl Into<Vec<u8>>) -> Self {
        Self {
            kind,
            buf: Arc::new(buf.into()),
        }
    }
}

/// A buffer for capturing `stdout`, `stderr` and info output.
#[derive(derive_more::Debug)]
pub struct Logger {
    #[debug("self.iter().collect::<Vec>()")]
    entries: LocalRb<Heap<LogData>>,
    partial: LogPartial,
}

impl Logger {
    /// Creates a new logger.
    pub fn new(entry_capacity: usize, partial_capacity: usize) -> Self {
        let entries = LocalRb::new(entry_capacity);
        let partial = LogPartial::new(partial_capacity);
        Self { entries, partial }
    }

    /// Returns the number of entries in the logger.
    pub fn len(&self) -> usize {
        self.entries.occupied_len()
    }

    /// Converts the logger to a vector of [`ratatui::text::Line`]s.
    pub fn to_lines(&self) -> Vec<ratatui::text::Line<'_>> {
        self::parse::to_lines(self)
    }

    /// Clears the logger.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.partial.clear();
    }

    /// Writes data to the logger.
    pub fn write_data(&mut self, mut data: &[u8], kind: LogKind) {
        while !data.is_empty() {
            if let Some(nl_pos) = find_newline_pos(data) {
                // Include the newline in this segment
                let (line_bytes, rest) = data.split_at(nl_pos + 1);
                self.extend_line(line_bytes, kind);
                self.push_partial(); // flush after newline
                data = rest;
            } else {
                // No newline in remaining data
                self.extend_line(data, kind);
                break;
            }
        }
    }

    /// Extends the partial buffer with the given data.
    ///
    /// If the partial buffer is full, it is flushed and pushed to the ring buffer.
    fn extend_line(&mut self, mut data: &[u8], kind: LogKind) {
        while !data.is_empty() {
            let leftover = self.partial.extend(data, kind);
            if self.partial.is_full() {
                self.push_partial();
            }
            data = leftover;
        }
    }

    /// Takes data from the partial buffer
    /// and pushes it to the ring buffer, overwriting data.
    ///
    /// This also clears the partial buffer.
    fn push_partial(&mut self) {
        if !self.partial.is_empty() {
            let entry = self.partial.flush().expect("partial is not empty");
            self.entries.push_overwrite(entry);
        }
    }
}

/// Finds the position of the first newline byte.
fn find_newline_pos(data: &[u8]) -> Option<usize> {
    data.iter().position(|&c| c == b'\n')
}

/// Normalizes line endings without allocating a new buffer.
pub fn normalize_newline(buf: &mut [u8]) -> &[u8] {
    let mut write = 0;
    let mut read = 0;
    while read < buf.len() {
        match buf[read] {
            b'\r' => {
                if read + 1 < buf.len() && buf[read + 1] == b'\n' {
                    buf[write] = b'\n'; // CRLF -> LF
                    write += 1;
                    read += 2; // skip both
                } else {
                    read += 1; // skip lone CR
                }
            }
            b => {
                if write != read {
                    buf[write] = b;
                }
                write += 1;
                read += 1;
            }
        }
    }
    &buf[..write]
}

#[cfg(test)]
mod tests {
    use ringbuf::traits::{Consumer as _, Observer as _};

    use super::*;

    #[test]
    fn test_write_without_newline() {
        let mut logger = Logger::new(10, 8);
        logger.write_data(b"hello", LogKind::Stdout);
        // Partial buffer should contain "hello"
        let partial_contents = logger.partial.contents().expect("partial is not empty").1;
        assert_eq!(partial_contents, b"hello");
        // Ring buffer should be empty
        assert!(logger.entries.is_empty());
    }

    #[test]
    fn test_write_with_newline() {
        let mut logger = Logger::new(10, 8);
        logger.write_data(b"hello\nworld", LogKind::Stdout);
        // After newline, partial buffer should contain "world"
        let partial_contents = logger.partial.contents().expect("partial is not empty").1;
        assert_eq!(partial_contents, b"world");
        // Ring buffer should contain "hello\n"
        let entry = logger.entries.pop_iter().next().expect("entry");
        assert_eq!(entry.kind, LogKind::Stdout);
        assert_eq!(entry.buf, Arc::new(b"hello\n".to_vec()));
    }

    #[test]
    fn test_partial_flush_on_capacity() {
        let mut logger = Logger::new(10, 5);
        logger.write_data(b"abcde", LogKind::Stdout); // fills partial buffer
        // Partial buffer should be empty after flush
        assert!(logger.partial.is_empty());
        // Ring buffer should contain "abcde"
        let entry = logger.entries.pop_iter().next().expect("entry");
        assert_eq!(entry.kind, LogKind::Stdout);
        assert_eq!(entry.buf, Arc::new(b"abcde".to_vec()));
    }

    #[test]
    fn test_multiple_lines_and_flushes() {
        let mut logger = Logger::new(10, 5);
        logger.write_data(b"a\nbcdefgh\nijkl\n", LogKind::Stderr);
        // Verify.
        let expected = ["a\n", "bcdef", "gh\n", "ijkl\n"];
        assert_eq!(logger.entries.occupied_len(), expected.len());
        logger
            .entries
            .pop_iter()
            .zip(expected)
            .for_each(|(entry, expected)| {
                assert_eq!(entry.kind, LogKind::Stderr);
                assert_eq!(entry.buf, Arc::new(expected.as_bytes().to_vec()));
            });
        // Partial buffer should be empty.
        assert!(logger.partial.is_empty());
    }

    #[test]
    fn test_ring_buffer_overwrite() {
        // Small ring buffer.
        let mut logger = Logger::new(3, 5);
        logger.write_data(b"one\n", LogKind::Stdout);
        logger.write_data(b"two\n", LogKind::Stdout);
        // "\n" from "three\n" should overwrite "one\n" by exceeding partial capacity.
        logger.write_data(b"three\n", LogKind::Stdout);
        // Verify.
        let expected = ["two\n", "three", "\n"];
        assert_eq!(logger.entries.occupied_len(), expected.len());
        logger
            .entries
            .pop_iter()
            .zip(expected)
            .for_each(|(entry, expected)| {
                let s = String::from_utf8_lossy(&entry.buf);
                assert_eq!(s, expected);
            });
    }
}
