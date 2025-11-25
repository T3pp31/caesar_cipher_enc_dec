//! # Decrypt Function Tests
//!
//! Tests for the `decrypt` function covering normal cases, boundary values, and abnormal cases.

use caesar_cipher_enc_dec::caesar_cipher::decrypt;

// =============================================================================
// Normal cases (正常系)
// =============================================================================

#[test]
fn test_decrypt_uppercase_only() {
    // Given: Encrypted uppercase text
    let text = "DEF";
    let shift = 3;

    // When: Decrypting
    let result = decrypt(text, shift);

    // Then: Returns original text
    assert_eq!(result, "ABC");
}

#[test]
fn test_decrypt_lowercase_only() {
    // Given: Encrypted lowercase text
    let text = "def";
    let shift = 3;

    // When: Decrypting
    let result = decrypt(text, shift);

    // Then: Returns original text
    assert_eq!(result, "abc");
}

#[test]
fn test_decrypt_with_symbols() {
    // Given: Encrypted text with symbols
    let text = "L ORYH BRX.";
    let shift = 3;

    // When: Decrypting
    let result = decrypt(text, shift);

    // Then: Returns original sentence
    assert_eq!(result, "I LOVE YOU.");
}

// =============================================================================
// Boundary values (境界値)
// =============================================================================

#[test]
fn test_decrypt_shift_zero() {
    // Given: Shift of 0
    let text = "ABC";
    let shift = 0;

    // When: Decrypting
    let result = decrypt(text, shift);

    // Then: Text unchanged
    assert_eq!(result, "ABC");
}

#[test]
fn test_decrypt_shift_25() {
    // Given: Shift of 25
    let text = "Z";
    let shift = 25;

    // When: Decrypting
    let result = decrypt(text, shift);

    // Then: Z becomes A
    assert_eq!(result, "A");
}

#[test]
fn test_decrypt_shift_26_wrap() {
    // Given: Shift of 26
    let text = "ABC";
    let shift = 26;

    // When: Decrypting
    let result = decrypt(text, shift);

    // Then: Text unchanged
    assert_eq!(result, "ABC");
}

#[test]
fn test_decrypt_negative_shift() {
    // Given: Negative shift (equivalent to positive encryption)
    let text = "A";
    let shift = -1;

    // When: Decrypting with -1 (same as encrypt with +1)
    let result = decrypt(text, shift);

    // Then: A becomes B
    assert_eq!(result, "B");
}

// =============================================================================
// Abnormal cases (異常系)
// =============================================================================

#[test]
fn test_decrypt_empty_string() {
    // Given: Empty string
    let text = "";
    let shift = 3;

    // When: Decrypting
    let result = decrypt(text, shift);

    // Then: Returns empty string
    assert_eq!(result, "");
}

#[test]
fn test_decrypt_non_ascii() {
    // Given: Non-ASCII text
    let text = "日本語";
    let shift = 5;

    // When: Decrypting
    let result = decrypt(text, shift);

    // Then: Characters unchanged
    assert_eq!(result, "日本語");
}
