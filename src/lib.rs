//! # Caesar Cipher Encryption/Decryption Library
//!
//! This library provides a comprehensive implementation of the Caesar cipher,
//! including both basic and safe (error-handling) encryption/decryption functions,
//! as well as a command-line interface for easy usage.
//!
//! ## Features
//!
//! - Basic encryption and decryption functions
//! - Safe versions with input validation and error handling
//! - Command-line interface with multiple operation modes
//! - Interactive mode for repeated operations
//! - Brute force decryption for unknown shift values
//! - Support for file input/output
//!
//! ## Quick Start
//!
//! ```rust
//! use caesar_cipher_enc_dec::caesar_cipher::{encrypt, decrypt};
//!
//! let original = "Hello World";
//! let encrypted = encrypt(original, 3);
//! let decrypted = decrypt(&encrypted, 3);
//! assert_eq!(original, decrypted);
//! ```
//!
//! ## Modules
//!
//! - [`config`] - Centralized constants and configuration
//! - [`caesar_cipher`] - Core encryption/decryption functionality
//! - [`cli`] - Command-line interface implementation
//!
//! ## API modes (library)
//!
//! | API | Shift range | Empty / whitespace-only text |
//! |-----|-------------|------------------------------|
//! | [`caesar_cipher::encrypt`] / [`caesar_cipher::decrypt`] | Any `i16` (normalized mod 26) | Allowed |
//! | [`caesar_cipher::encrypt_safe`] / [`caesar_cipher::decrypt_safe`] | -25 to 25 only | `CipherError::EmptyText` |
//!
//! Use the `*_safe` functions when you want validation errors instead of silent normalization.

/// Bounded stdin/file reading used by the CLI
mod bounded_input;

/// Centralized configuration and constants
pub mod config;

/// Core Caesar cipher encryption and decryption functionality
pub mod caesar_cipher;

/// Command-line interface implementation
pub mod cli;
