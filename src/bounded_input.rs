//! Bounded input reading for CLI and interactive mode.
//!
//! Reads stdin lines and files with a hard byte limit so untrusted input
//! cannot exhaust memory before validation runs.

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/// Chunk size for bounded line reads (not the input cap; keeps I/O efficient up to `max_bytes`).
const READ_CHUNK_SIZE: usize = 8192;

/// Returns a consistent error message when input exceeds `max_bytes`.
pub fn input_size_exceeded_message(max_bytes: usize) -> String {
    format!("Input exceeds maximum size of {} bytes", max_bytes)
}

/// Reads a single line from `reader`, stopping at newline or EOF.
///
/// At most `max_bytes` bytes of **line content** may appear before the newline.
/// The trailing `\n` (if present) is stored but does not count toward `max_bytes`.
/// If a non-newline byte would push content past `max_bytes`, returns an error.
pub fn read_line_bounded<R: Read>(reader: &mut R, max_bytes: usize) -> io::Result<String> {
    let mut line = Vec::with_capacity(max_bytes.min(READ_CHUNK_SIZE));
    let mut buf = [0u8; READ_CHUNK_SIZE];

    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        for &byte in &buf[..n] {
            if byte == b'\n' {
                line.push(byte);
                return String::from_utf8(line)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e));
            }
            if line.len() >= max_bytes {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    input_size_exceeded_message(max_bytes),
                ));
            }
            line.push(byte);
        }
    }

    String::from_utf8(line).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

/// Reads a regular file up to `max_bytes`, with streaming size enforcement.
///
/// Rejects non-regular files (directories, devices, FIFOs, etc.) and stops
/// reading once more than `max_bytes` have been read (TOCTOU-safe).
pub fn read_file_bounded(path: impl AsRef<Path>, max_bytes: usize) -> io::Result<String> {
    let path = path.as_ref();
    let path_display = path.display().to_string();

    let metadata = std::fs::metadata(path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Failed to read file '{}': {}", path_display, e),
        )
    })?;

    if !metadata.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Input path '{}' is not a regular file", path_display),
        ));
    }

    if metadata.len() > max_bytes as u64 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "Input file '{}' exceeds maximum size of {} bytes",
                path_display, max_bytes
            ),
        ));
    }

    let file = File::open(path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Failed to read file '{}': {}", path_display, e),
        )
    })?;

    let mut limited = file.take(max_bytes as u64 + 1);
    let mut bytes = Vec::new();
    limited.read_to_end(&mut bytes)?;

    if bytes.len() > max_bytes {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "Input file '{}' exceeds maximum size of {} bytes",
                path_display, max_bytes
            ),
        ));
    }

    String::from_utf8(bytes).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Input file '{}' is not valid UTF-8: {}", path_display, e),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn read_line_bounded_accepts_line_within_limit() {
        // Given: A line shorter than the limit
        let mut reader = Cursor::new(b"hello\n");

        // When: Reading with a generous limit
        let result = read_line_bounded(&mut reader, 10);

        // Then: Returns content including newline
        assert_eq!(result.unwrap(), "hello\n");
    }

    #[test]
    fn read_line_bounded_rejects_oversized_line() {
        // Given: A line longer than the limit (no early stop on allocation)
        let mut reader = Cursor::new(b"abcdef\n");

        // When: Limit is 3 bytes
        let result = read_line_bounded(&mut reader, 3);

        // Then: Fails with size error
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("exceeds maximum size"));
    }

    #[test]
    fn read_line_bounded_eof_without_newline() {
        // Given: Input without trailing newline
        let mut reader = Cursor::new(b"hi");

        // When: Reading until EOF
        let result = read_line_bounded(&mut reader, 10);

        // Then: Returns bytes read
        assert_eq!(result.unwrap(), "hi");
    }

    #[test]
    fn read_line_bounded_accepts_exactly_max_bytes_at_eof() {
        // Given: Exactly max_bytes of content with no trailing newline
        let payload = vec![b'x'; 5];
        let mut reader = Cursor::new(payload);

        // When: Limit equals payload length
        let result = read_line_bounded(&mut reader, 5);

        // Then: Succeeds (matches --text / file cap semantics for payload size)
        assert_eq!(result.unwrap(), "xxxxx");
    }

    #[test]
    fn read_line_bounded_allows_content_plus_newline_at_limit() {
        // Given: max_bytes of content followed by newline (newline not counted toward cap)
        let mut payload = vec![b'a'; 3];
        payload.push(b'\n');
        let mut reader = Cursor::new(payload);

        // When: Limit equals content length
        let result = read_line_bounded(&mut reader, 3);

        // Then: Succeeds; newline is stored but not counted toward max_bytes
        assert_eq!(result.unwrap(), "aaa\n");
    }

    #[test]
    fn read_line_bounded_rejects_one_byte_over_content_limit() {
        // Given: Content one byte over the limit, no newline
        let payload = vec![b'b'; 4];
        let mut reader = Cursor::new(payload);

        // When: Limit is 3
        let result = read_line_bounded(&mut reader, 3);

        // Then: Fails on the fourth byte
        assert!(result.is_err());
    }
}
