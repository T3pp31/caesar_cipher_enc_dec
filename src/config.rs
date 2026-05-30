//! # Configuration Module
//!
//! Centralized constants and configuration values for the Caesar cipher application.
//! All configurable values are defined here to avoid hardcoding in individual modules.

/// Size of the alphabet (A-Z)
pub const ALPHABET_SIZE: i16 = 26;

/// Maximum valid shift value for safe functions
pub const MAX_SHIFT: i16 = 25;

/// Minimum valid shift value for safe functions
pub const MIN_SHIFT: i16 = -25;

/// ASCII value of uppercase 'A'
pub const UPPERCASE_BASE: i16 = 'A' as i16;

/// ASCII value of lowercase 'a'
pub const LOWERCASE_BASE: i16 = 'a' as i16;

/// Maximum shift value for brute force decryption
pub const MAX_BRUTE_FORCE_SHIFT: i16 = 25;

/// Default shift value when user input is invalid
pub const DEFAULT_SHIFT: i16 = 3;

/// Maximum input size in bytes (10 MB)
pub const MAX_INPUT_SIZE: usize = 10 * 1024 * 1024;

/// Maximum bytes for a single interactive shift prompt line
pub const MAX_SHIFT_LINE_SIZE: usize = 64;

// Compile-time checks for configuration relationships (see also tests/config_tests.rs).
const _: () = {
    assert!(UPPERCASE_BASE < LOWERCASE_BASE);
    assert!(MAX_SHIFT < ALPHABET_SIZE);
    assert!(DEFAULT_SHIFT >= 1 && DEFAULT_SHIFT <= MAX_SHIFT);
    assert!(ALPHABET_SIZE > 0);
    assert!(MAX_SHIFT > 0);
    assert!(UPPERCASE_BASE > 0);
    assert!(LOWERCASE_BASE > 0);
    assert!(MAX_BRUTE_FORCE_SHIFT > 0);
    assert!(DEFAULT_SHIFT > 0);
    assert!(UPPERCASE_BASE >= 0 && UPPERCASE_BASE <= 127);
    assert!(LOWERCASE_BASE >= 0 && LOWERCASE_BASE <= 127);
};
