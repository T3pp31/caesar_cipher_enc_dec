//! # Configuration Module
//!
//! Centralized constants and configuration values for the Caesar cipher application.
//! All configurable values are defined here to avoid hardcoding in individual modules.

/// Size of the alphabet (A-Z)
pub const ALPHABET_SIZE: i16 = 26;

/// Maximum valid shift value for safe functions
pub const MAX_SHIFT: i16 = 25;

/// ASCII value of uppercase 'A'
pub const UPPERCASE_BASE: i16 = 'A' as i16;

/// ASCII value of lowercase 'a'
pub const LOWERCASE_BASE: i16 = 'a' as i16;

/// Maximum shift value for brute force decryption
pub const MAX_BRUTE_FORCE_SHIFT: i16 = 25;

/// Default shift value when user input is invalid
pub const DEFAULT_SHIFT: i16 = 3;
