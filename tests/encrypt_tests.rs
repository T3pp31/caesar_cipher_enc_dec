//! # Encrypt Function Tests
//!
//! Tests for the `encrypt` function covering normal cases, boundary values, and abnormal cases.

use caesar_cipher_enc_dec::caesar_cipher::encrypt;

// =============================================================================
// Normal cases (正常系)
// =============================================================================

#[test]
fn test_encrypt_uppercase_only() {
    // Given: Uppercase text with shift 3
    let text = "ABC";
    let shift = 3;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: Each character shifted by 3
    assert_eq!(result, "DEF");
}

#[test]
fn test_encrypt_lowercase_only() {
    // Given: Lowercase text with shift 3
    let text = "abc";
    let shift = 3;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: Each character shifted by 3
    assert_eq!(result, "def");
}

#[test]
fn test_encrypt_mixed_case() {
    // Given: Mixed case text with shift 1
    let text = "AbCdEf";
    let shift = 1;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: Case preserved, each letter shifted
    assert_eq!(result, "BcDeFg");
}

#[test]
fn test_encrypt_with_numbers_and_symbols() {
    // Given: Text containing numbers and symbols
    let text = "A1!B2@C3#";
    let shift = 1;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: Only letters shifted, numbers and symbols unchanged
    assert_eq!(result, "B1!C2@D3#");
}

#[test]
fn test_encrypt_with_spaces() {
    // Given: Text with spaces
    let text = "Hello World";
    let shift = 3;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: Spaces preserved
    assert_eq!(result, "Khoor Zruog");
}

#[test]
fn test_encrypt_full_sentence() {
    // Given: A complete sentence
    let text = "I LOVE YOU.";
    let shift = 3;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: Correctly encrypted
    assert_eq!(result, "L ORYH BRX.");
}

// =============================================================================
// Boundary values (境界値)
// =============================================================================

#[test]
fn test_encrypt_shift_zero() {
    // Given: Shift value of 0 (identity)
    let text = "ABC";
    let shift = 0;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: Text unchanged
    assert_eq!(result, "ABC");
}

#[test]
fn test_encrypt_shift_one_minimum_positive() {
    // Given: Minimum positive shift (1)
    let text = "A";
    let shift = 1;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: A becomes B
    assert_eq!(result, "B");
}

#[test]
fn test_encrypt_shift_25_maximum_effective() {
    // Given: Maximum effective shift (25)
    let text = "A";
    let shift = 25;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: A becomes Z
    assert_eq!(result, "Z");
}

#[test]
fn test_encrypt_shift_26_full_wrap() {
    // Given: Shift of 26 (full alphabet wrap)
    let text = "ABC";
    let shift = 26;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: Text unchanged (26 mod 26 = 0)
    assert_eq!(result, "ABC");
}

#[test]
fn test_encrypt_shift_27_wrap_plus_one() {
    // Given: Shift of 27 (wrap + 1)
    let text = "ABC";
    let shift = 27;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: Same as shift 1
    assert_eq!(result, "BCD");
}

#[test]
fn test_encrypt_shift_negative_one() {
    // Given: Negative shift of -1
    let text = "B";
    let shift = -1;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: B becomes A
    assert_eq!(result, "A");
}

#[test]
fn test_encrypt_shift_negative_25() {
    // Given: Negative shift of -25
    let text = "Z";
    let shift = -25;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: Z becomes A
    assert_eq!(result, "A");
}

#[test]
fn test_encrypt_shift_negative_26_wrap() {
    // Given: Negative shift of -26 (full wrap)
    let text = "ABC";
    let shift = -26;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: Text unchanged
    assert_eq!(result, "ABC");
}

#[test]
fn test_encrypt_alphabet_boundary_a() {
    // Given: Letter 'A' at alphabet start
    let text = "A";
    let shift = 1;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: A becomes B
    assert_eq!(result, "B");
}

#[test]
fn test_encrypt_alphabet_boundary_z_wrap() {
    // Given: Letter 'Z' at alphabet end
    let text = "Z";
    let shift = 1;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: Z wraps to A
    assert_eq!(result, "A");
}

#[test]
fn test_encrypt_lowercase_boundary_z_wrap() {
    // Given: Lowercase 'z' at alphabet end
    let text = "z";
    let shift = 1;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: z wraps to a
    assert_eq!(result, "a");
}

#[test]
fn test_encrypt_lowercase_boundary_a_negative() {
    // Given: Lowercase 'a' with negative shift
    let text = "a";
    let shift = -1;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: a wraps to z
    assert_eq!(result, "z");
}

// =============================================================================
// Abnormal cases (異常系)
// =============================================================================

#[test]
fn test_encrypt_empty_string() {
    // Given: Empty string
    let text = "";
    let shift = 3;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: Returns empty string
    assert_eq!(result, "");
}

#[test]
fn test_encrypt_non_ascii_characters_only() {
    // Given: Japanese characters only
    let text = "あいうえお";
    let shift = 3;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: Characters unchanged
    assert_eq!(result, "あいうえお");
}

#[test]
fn test_encrypt_mixed_ascii_and_japanese() {
    // Given: Mixed ASCII and Japanese
    let text = "Hello世界";
    let shift = 3;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: Only ASCII letters shifted
    assert_eq!(result, "Khoor世界");
}

#[test]
fn test_encrypt_extreme_positive_shift() {
    // Given: Extreme positive shift (i16::MAX)
    let text = "A";
    let shift = i16::MAX;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: Shift normalized via rem_euclid
    // i16::MAX = 32767, 32767 % 26 = 7
    assert_eq!(result, "H");
}

#[test]
fn test_encrypt_extreme_negative_shift() {
    // Given: Extreme negative shift (i16::MIN)
    let text = "A";
    let shift = i16::MIN;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: Shift normalized via rem_euclid
    // i16::MIN = -32768, rem_euclid(-32768, 26) = 18
    assert_eq!(result, "S");
}

#[test]
fn test_encrypt_numbers_only() {
    // Given: Numbers only
    let text = "12345";
    let shift = 5;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: Numbers unchanged
    assert_eq!(result, "12345");
}

#[test]
fn test_encrypt_special_characters_only() {
    // Given: Special characters only
    let text = "!@#$%^&*()";
    let shift = 10;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: Special characters unchanged
    assert_eq!(result, "!@#$%^&*()");
}

#[test]
fn test_encrypt_emoji() {
    // Given: Text with emoji
    let text = "Hello 😀";
    let shift = 3;

    // When: Encrypting
    let result = encrypt(text, shift);

    // Then: Emoji unchanged
    assert_eq!(result, "Khoor 😀");
}
