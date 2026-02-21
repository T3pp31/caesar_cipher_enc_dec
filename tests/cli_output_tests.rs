//! # CLI Output and Input Tests
//!
//! Tests for CLI helper functions including file I/O with improved error messages.
//!
//! ## テスト観点表（等価分割・境界値）
//!
//! | テスト観点                       | 分類     | 期待値                                |
//! |----------------------------------|----------|---------------------------------------|
//! | 存在しないファイル読込           | 異常系   | エラーメッセージにファイルパス含む    |
//! | text+file同時指定                | 異常系   | "Cannot specify both" エラー          |
//! | テキスト引数のみ指定             | 正常系   | テキストがそのまま返る                |
//! | ファイルから正常読込             | 正常系   | ファイル内容が返る                    |
//! | 空ファイル読込                   | 境界値   | 空文字列返却                          |
//! | ファイルへの正常出力             | 正常系   | ファイルに正しく書き込まれる          |
//! | 大きなテキストの出力             | 境界値   | 正常に書き込まれる                    |
//! | 特殊文字を含むテキストの出力     | 正常系   | そのまま書き込まれる                  |
//! | 存在しないディレクトリへの出力   | 異常系   | エラー返却                            |
//! | 存在しないファイルのエラーメッセージ検証 | 異常系 | パスが含まれること              |
//! | Unicode テキストのファイル読み書き | 正常系 | UTF-8が保持される                    |
//! | 空テキスト引数                   | 境界値   | 空文字列がそのまま返る                |

use std::fs;
use std::io::Write;
use tempfile::{NamedTempFile, TempDir};

// Note: get_input_text and output_result are private functions in cli module.
// We test them indirectly through the CLI binary or test the public interface.
// For direct testing, we use the test module within cli.rs.
// Here we test the CLI binary behavior via process execution.

// =============================================================================
// CLI binary integration tests
// =============================================================================

#[test]
fn test_cli_encrypt_with_text_arg() {
    // Given: CLI with encrypt command and text argument
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "encrypt", "--text", "Hello", "--shift", "3"])
        .output()
        .expect("Failed to execute CLI");

    // When: Command executes
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Then: Output contains encrypted text
    assert!(
        stdout.contains("Khoor"),
        "Expected 'Khoor' in output, got: {}",
        stdout
    );
}

#[test]
fn test_cli_decrypt_with_text_arg() {
    // Given: CLI with decrypt command and text argument
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "decrypt", "--text", "Khoor", "--shift", "3"])
        .output()
        .expect("Failed to execute CLI");

    // When: Command executes
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Then: Output contains decrypted text
    assert!(
        stdout.contains("Hello"),
        "Expected 'Hello' in output, got: {}",
        stdout
    );
}

#[test]
fn test_cli_encrypt_from_file() {
    // Given: A temporary file with text content
    let mut temp_file = NamedTempFile::new().unwrap();
    write!(temp_file, "Hello World").unwrap();
    let file_path = temp_file.path().to_string_lossy().to_string();

    // When: CLI reads from file
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "encrypt", "--file", &file_path, "--shift", "3"])
        .output()
        .expect("Failed to execute CLI");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Then: Output contains encrypted text
    assert!(
        stdout.contains("Khoor Zruog"),
        "Expected 'Khoor Zruog' in output, got: {}",
        stdout
    );
}

#[test]
fn test_cli_encrypt_output_to_file() {
    // Given: A temporary output file path
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("output.txt");
    let output_path_str = output_path.to_string_lossy().to_string();

    // When: CLI writes to file
    let result = std::process::Command::new("cargo")
        .args([
            "run",
            "--",
            "encrypt",
            "--text",
            "Hello",
            "--shift",
            "3",
            "--output",
            &output_path_str,
        ])
        .output()
        .expect("Failed to execute CLI");

    let stdout = String::from_utf8_lossy(&result.stdout);

    // Then: File contains encrypted text and message includes path
    let content = fs::read_to_string(&output_path).unwrap();
    assert_eq!(content, "Khoor");
    assert!(
        stdout.contains(&output_path_str),
        "Output message should contain file path '{}', got: {}",
        output_path_str,
        stdout
    );
}

// =============================================================================
// Error handling tests (異常系)
// =============================================================================

#[test]
fn test_cli_nonexistent_file_error_contains_path() {
    // Given: A path to a nonexistent file
    let nonexistent_path = "/tmp/nonexistent_caesar_test_file_12345.txt";

    // When: CLI tries to read from nonexistent file
    let output = std::process::Command::new("cargo")
        .args([
            "run",
            "--",
            "encrypt",
            "--file",
            nonexistent_path,
            "--shift",
            "3",
        ])
        .output()
        .expect("Failed to execute CLI");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Then: Error message contains the file path
    assert!(
        stderr.contains(nonexistent_path),
        "Error message should contain file path '{}', got: {}",
        nonexistent_path,
        stderr
    );
}

#[test]
fn test_cli_both_text_and_file_error() {
    // Given: Both text and file arguments provided
    let mut temp_file = NamedTempFile::new().unwrap();
    write!(temp_file, "content").unwrap();
    let file_path = temp_file.path().to_string_lossy().to_string();

    // When: CLI receives both arguments
    let output = std::process::Command::new("cargo")
        .args([
            "run", "--", "encrypt", "--text", "Hello", "--file", &file_path, "--shift", "3",
        ])
        .output()
        .expect("Failed to execute CLI");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Then: Error message mentions the conflict
    assert!(
        stderr.contains("Cannot specify both"),
        "Error should mention 'Cannot specify both', got: {}",
        stderr
    );
}

// =============================================================================
// Boundary value tests (境界値)
// =============================================================================

#[test]
fn test_cli_encrypt_empty_file() {
    // Given: An empty temporary file
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_string_lossy().to_string();

    // When: CLI reads from empty file
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "encrypt", "--file", &file_path, "--shift", "3"])
        .output()
        .expect("Failed to execute CLI");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Then: Output should be empty (just a newline from println)
    assert!(
        stdout.trim().is_empty(),
        "Expected empty output for empty file, got: '{}'",
        stdout.trim()
    );
}

#[test]
fn test_cli_brute_force_output() {
    // Given: An encrypted text
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "brute-force", "--text", "Khoor"])
        .output()
        .expect("Failed to execute CLI");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Then: Output contains brute force header and shift 3 shows "Hello"
    assert!(
        stdout.contains("Brute Force Decryption"),
        "Should contain brute force header, got: {}",
        stdout
    );
    assert!(
        stdout.contains("Shift  3: Hello"),
        "Shift 3 should decrypt to 'Hello', got: {}",
        stdout
    );
}

#[test]
fn test_cli_version_matches_cargo_toml() {
    // Given: CLI with --version flag
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "--version"])
        .output()
        .expect("Failed to execute CLI");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Then: Version should match Cargo.toml (dynamically read at compile time)
    let expected_version = env!("CARGO_PKG_VERSION");
    assert!(
        stdout.contains(expected_version),
        "CLI version should be {} from Cargo.toml, got: {}",
        expected_version,
        stdout
    );
}

#[test]
fn test_cli_encrypt_unicode_text() {
    // Given: Unicode text (Japanese + English mix)
    let output = std::process::Command::new("cargo")
        .args([
            "run",
            "--",
            "encrypt",
            "--text",
            "Hello世界",
            "--shift",
            "3",
        ])
        .output()
        .expect("Failed to execute CLI");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Then: Only English letters shifted, Japanese preserved
    assert!(
        stdout.contains("Khoor世界"),
        "Expected 'Khoor世界' in output, got: {}",
        stdout
    );
}

#[test]
fn test_cli_safe_mode_invalid_shift_error() {
    // Given: CLI with safe mode and invalid shift value
    let output = std::process::Command::new("cargo")
        .args([
            "run", "--", "encrypt", "--text", "Hello", "--shift", "30", "--safe",
        ])
        .output()
        .expect("Failed to execute CLI");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Then: Error message about invalid shift
    assert!(
        stderr.contains("Invalid shift value") || stderr.contains("out of range"),
        "Should show shift validation error, got: {}",
        stderr
    );
}

#[test]
fn test_cli_safe_mode_empty_text_from_file() {
    // Given: Empty file with safe mode
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_string_lossy().to_string();

    // When: CLI reads empty file in safe mode
    let output = std::process::Command::new("cargo")
        .args([
            "run", "--", "encrypt", "--file", &file_path, "--shift", "3", "--safe",
        ])
        .output()
        .expect("Failed to execute CLI");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Then: Error about empty text
    assert!(
        stderr.contains("empty"),
        "Should show empty text error, got: {}",
        stderr
    );
}

// =============================================================================
// Brute force shift=0 tests
// =============================================================================

#[test]
fn test_cli_brute_force_includes_shift_zero() {
    // Given: An encrypted text
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "brute-force", "--text", "Khoor"])
        .output()
        .expect("Failed to execute CLI");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Then: Output contains shift 0
    assert!(
        stdout.contains("Shift  0:"),
        "Brute force output should include shift 0, got: {}",
        stdout
    );
}

#[test]
fn test_cli_brute_force_shift_zero_is_original() {
    // Given: A known encrypted text "Khoor"
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "brute-force", "--text", "Khoor"])
        .output()
        .expect("Failed to execute CLI");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Then: Shift 0 shows the original text unchanged
    assert!(
        stdout.contains("Shift  0: Khoor"),
        "Shift 0 should show original text 'Khoor', got: {}",
        stdout
    );
}

// =============================================================================
// Help text tests
// =============================================================================

#[test]
fn test_cli_encrypt_help_shows_correct_shift_description() {
    // Given: CLI with encrypt --help
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "encrypt", "--help"])
        .output()
        .expect("Failed to execute CLI");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Then: Help text should describe shift range accurately
    assert!(
        stdout.contains("safe mode: -25 to 25"),
        "Help text should mention safe mode range, got: {}",
        stdout
    );
    assert!(
        !stdout.contains("Shift value (1-25)"),
        "Help text should NOT contain outdated '1-25' range, got: {}",
        stdout
    );
}
