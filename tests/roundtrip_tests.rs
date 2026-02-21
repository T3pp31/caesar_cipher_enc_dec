//! # Roundtrip Tests
//!
//! Tests that verify encryption followed by decryption returns the original text.

use caesar_cipher_enc_dec::caesar_cipher::{decrypt, decrypt_safe, encrypt, encrypt_safe};

// =============================================================================
// Basic roundtrip tests
// =============================================================================

#[test]
fn test_roundtrip_simple() {
    // Given: Simple text
    let original = "Hello";
    let shift = 5;

    // When: Encrypt then decrypt
    let encrypted = encrypt(original, shift);
    let decrypted = decrypt(&encrypted, shift);

    // Then: Returns original
    assert_eq!(original, decrypted);
}

#[test]
fn test_roundtrip_full_sentence() {
    // Given: Full sentence with punctuation
    let original = "The quick brown fox jumps over the lazy dog!";
    let shift = 13;

    // When: Encrypt then decrypt
    let encrypted = encrypt(original, shift);
    let decrypted = decrypt(&encrypted, shift);

    // Then: Returns original
    assert_eq!(original, decrypted);
}

#[test]
fn test_roundtrip_all_shifts() {
    // Given: Test text
    let original = "TestString";

    // When/Then: All valid shifts produce correct roundtrip
    for shift in 0..=25 {
        let encrypted = encrypt(original, shift);
        let decrypted = decrypt(&encrypted, shift);
        assert_eq!(original, decrypted, "Failed for shift {}", shift);
    }
}

#[test]
fn test_roundtrip_negative_shifts() {
    // Given: Test text
    let original = "NegativeTest";

    // When/Then: Negative shifts work correctly
    for shift in -25..=0 {
        let encrypted = encrypt(original, shift);
        let decrypted = decrypt(&encrypted, shift);
        assert_eq!(original, decrypted, "Failed for shift {}", shift);
    }
}

#[test]
fn test_roundtrip_using_negative_encrypt() {
    // Given: Original text
    let original = "Alternative";
    let shift = 7;

    // When: Encrypt then decrypt using negative encrypt
    let encrypted = encrypt(original, shift);
    let decrypted = encrypt(&encrypted, -shift);

    // Then: Returns original
    assert_eq!(original, decrypted);
}

// =============================================================================
// Safe API roundtrip tests
// =============================================================================

#[test]
fn test_safe_roundtrip_valid() {
    // Given: Valid text and shift
    let original = "Hello World";
    let shift = 10;

    // When: Safe encrypt then decrypt
    let encrypted = encrypt_safe(original, shift).unwrap();
    let decrypted = decrypt_safe(&encrypted, shift).unwrap();

    // Then: Returns original
    assert_eq!(original, decrypted);
}

#[test]
fn test_safe_roundtrip_all_valid_shifts() {
    // Given: Test text
    let original = "RoundtripTest";

    // When/Then: All valid shifts produce correct roundtrip
    for shift in -25..=25 {
        let encrypted = encrypt_safe(original, shift).unwrap();
        let decrypted = decrypt_safe(&encrypted, shift).unwrap();
        assert_eq!(original, decrypted, "Failed for shift {}", shift);
    }
}
