//! Integration tests for bounded file input and CLI rejection paths.

use caesar_cipher_enc_dec::config::MAX_INPUT_SIZE;
use std::io::Write;
use tempfile::{NamedTempFile, TempDir};

#[test]
fn cli_rejects_directory_as_input_file() {
    // Given: A directory path
    let dir = TempDir::new().unwrap();
    let path = dir.path().to_string_lossy();

    // When: Encrypt with --file pointing at the directory
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_caesar_cipher_enc_dec"))
        .args(["encrypt", "--file", &path, "--shift", "3"])
        .output()
        .expect("failed to run binary");

    // Then: Fails with a clear message
    assert!(!output.status.success());
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(
        combined.contains("not a regular file"),
        "expected directory rejection, got: {}",
        combined
    );
}

#[test]
fn cli_accepts_file_at_max_input_size() {
    // Given: A file exactly at MAX_INPUT_SIZE
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(&vec![b'A'; MAX_INPUT_SIZE]).unwrap();
    temp_file.flush().unwrap();

    // When: Encrypt from file
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_caesar_cipher_enc_dec"))
        .args([
            "encrypt",
            "--file",
            temp_file.path().to_str().unwrap(),
            "--shift",
            "1",
        ])
        .output()
        .expect("failed to run binary");

    // Then: Succeeds
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn cli_rejects_file_over_max_input_size() {
    // Given: A file one byte over the limit
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file
        .write_all(&vec![b'B'; MAX_INPUT_SIZE + 1])
        .unwrap();
    temp_file.flush().unwrap();

    // When: Encrypt from file
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_caesar_cipher_enc_dec"))
        .args([
            "encrypt",
            "--file",
            temp_file.path().to_str().unwrap(),
            "--shift",
            "1",
        ])
        .output()
        .expect("failed to run binary");

    // Then: Fails with size message
    assert!(!output.status.success());
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(combined.contains("exceeds maximum size"));
}
