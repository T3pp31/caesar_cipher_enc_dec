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
//! - [`caesar_cipher`] - Core encryption/decryption functionality
//! - [`cli`] - Command-line interface implementation

/// Core Caesar cipher encryption and decryption functionality
pub mod caesar_cipher;

/// Command-line interface implementation
pub mod cli;
