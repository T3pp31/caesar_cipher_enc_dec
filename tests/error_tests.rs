//! # CipherError Tests
//!
//! Tests for the `CipherError` enum, including Display, Clone, PartialEq, and Debug implementations.

use caesar_cipher_enc_dec::caesar_cipher::CipherError;

#[test]
fn test_cipher_error_display_empty_text() {
    // Given: EmptyText error
    let error = CipherError::EmptyText;

    // When: Converting to string
    let message = error.to_string();

    // Then: Correct message
    assert_eq!(message, "Input text cannot be empty or whitespace-only");
}

#[test]
fn test_cipher_error_display_invalid_shift() {
    // Given: InvalidShift error with message
    let error = CipherError::InvalidShift("test message".to_string());

    // When: Converting to string
    let message = error.to_string();

    // Then: Correct message format
    assert_eq!(message, "Invalid shift value: test message");
}

#[test]
fn test_cipher_error_clone() {
    // Given: EmptyText error
    let original = CipherError::EmptyText;

    // When: Cloning
    let cloned = original.clone();

    // Then: Clone equals original
    assert_eq!(original, cloned);
}

#[test]
fn test_cipher_error_clone_invalid_shift() {
    // Given: InvalidShift error
    let original = CipherError::InvalidShift("test".to_string());

    // When: Cloning
    let cloned = original.clone();

    // Then: Clone equals original
    assert_eq!(original, cloned);
}

#[test]
fn test_cipher_error_partial_eq_same() {
    // Given: Two EmptyText errors
    let error1 = CipherError::EmptyText;
    let error2 = CipherError::EmptyText;

    // When/Then: They are equal
    assert_eq!(error1, error2);
}

#[test]
fn test_cipher_error_partial_eq_different_variants() {
    // Given: Different error variants
    let error1 = CipherError::EmptyText;
    let error2 = CipherError::InvalidShift("test".to_string());

    // When/Then: They are not equal
    assert_ne!(error1, error2);
}

#[test]
fn test_cipher_error_partial_eq_different_messages() {
    // Given: InvalidShift with different messages
    let error1 = CipherError::InvalidShift("message1".to_string());
    let error2 = CipherError::InvalidShift("message2".to_string());

    // When/Then: They are not equal
    assert_ne!(error1, error2);
}

#[test]
fn test_cipher_error_debug() {
    // Given: EmptyText error
    let error = CipherError::EmptyText;

    // When: Debug formatting
    let debug_str = format!("{:?}", error);

    // Then: Contains variant name
    assert!(debug_str.contains("EmptyText"));
}

#[test]
fn test_cipher_error_is_std_error() {
    // Given: A CipherError
    let error: Box<dyn std::error::Error> = Box::new(CipherError::EmptyText);

    // When/Then: Can be used as std::error::Error
    assert!(!error.to_string().is_empty());
}
