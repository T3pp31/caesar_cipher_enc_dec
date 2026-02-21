//! # decrypt_safe Overflow Prevention Tests
//!
//! Tests to verify that `decrypt_safe` handles extreme shift values safely
//! without integer overflow or panic.
//!
//! ## テスト観点表（等価分割・境界値）
//!
//! | テスト観点                     | 分類     | 期待値                                |
//! |--------------------------------|----------|---------------------------------------|
//! | shift=25 (最大有効値)          | 境界値   | Ok返却                                |
//! | shift=-25 (最小有効値)         | 境界値   | Ok返却                                |
//! | shift=26 (境界外+1)            | 境界値   | Err(InvalidShift)                     |
//! | shift=-26 (境界外-1)           | 境界値   | Err(InvalidShift)                     |
//! | shift=0                        | 境界値   | Ok返却、テキスト不変                  |
//! | shift=i16::MAX                 | 異常系   | Err(InvalidShift)、パニックしない     |
//! | shift=i16::MIN                 | 異常系   | Err(InvalidShift)、パニックしない     |
//! | 空テキスト+i16::MIN            | 異常系   | Err(EmptyText)                        |
//! | 空テキスト+i16::MAX            | 異常系   | Err(EmptyText)                        |
//! | shift=i16::MAX エラーメッセージ | 異常系  | エラーにシフト値が含まれる            |
//! | shift=i16::MIN エラーメッセージ | 異常系  | エラーにシフト値が含まれる            |
//! | 正常値での往復検証             | 正常系   | encrypt_safe→decrypt_safeで復元       |

use caesar_cipher_enc_dec::caesar_cipher::{decrypt_safe, encrypt_safe, CipherError};

// =============================================================================
// Boundary values (境界値)
// =============================================================================

#[test]
fn test_decrypt_safe_shift_25_maximum_valid() {
    // Given: Maximum valid shift value (25)
    let text = "Z";
    let shift = 25;

    // When: Decrypting safely
    let result = decrypt_safe(text, shift);

    // Then: Returns Ok
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "A");
}

#[test]
fn test_decrypt_safe_shift_negative_25_minimum_valid() {
    // Given: Minimum valid shift value (-25)
    let text = "A";
    let shift = -25;

    // When: Decrypting safely
    let result = decrypt_safe(text, shift);

    // Then: Returns Ok
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Z");
}

#[test]
fn test_decrypt_safe_shift_26_boundary_invalid() {
    // Given: Shift just outside valid range (26)
    let text = "Test";
    let shift = 26;

    // When: Decrypting safely
    let result = decrypt_safe(text, shift);

    // Then: Returns Err(InvalidShift)
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), CipherError::InvalidShift(_)));
}

#[test]
fn test_decrypt_safe_shift_negative_26_boundary_invalid() {
    // Given: Shift just outside valid range (-26)
    let text = "Test";
    let shift = -26;

    // When: Decrypting safely
    let result = decrypt_safe(text, shift);

    // Then: Returns Err(InvalidShift)
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), CipherError::InvalidShift(_)));
}

#[test]
fn test_decrypt_safe_shift_zero() {
    // Given: Zero shift (identity operation)
    let text = "Hello World";
    let shift = 0;

    // When: Decrypting safely
    let result = decrypt_safe(text, shift);

    // Then: Returns Ok with unchanged text
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello World");
}

// =============================================================================
// Abnormal cases (異常系) - Overflow prevention
// =============================================================================

#[test]
fn test_decrypt_safe_shift_i16_max_no_panic() {
    // Given: i16::MAX shift value (32767) - would cause overflow if negated unchecked
    let text = "Test";
    let shift = i16::MAX;

    // When: Decrypting safely
    let result = decrypt_safe(text, shift);

    // Then: Returns Err(InvalidShift) without panicking
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), CipherError::InvalidShift(_)));
}

#[test]
fn test_decrypt_safe_shift_i16_min_no_panic() {
    // Given: i16::MIN shift value (-32768) - negating this would overflow i16
    let text = "Test";
    let shift = i16::MIN;

    // When: Decrypting safely
    let result = decrypt_safe(text, shift);

    // Then: Returns Err(InvalidShift) without panicking
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), CipherError::InvalidShift(_)));
}

#[test]
fn test_decrypt_safe_empty_text_with_i16_min() {
    // Given: Empty text with i16::MIN shift
    let text = "";
    let shift = i16::MIN;

    // When: Decrypting safely
    let result = decrypt_safe(text, shift);

    // Then: Returns Err(EmptyText) - empty text check comes first
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), CipherError::EmptyText));
}

#[test]
fn test_decrypt_safe_empty_text_with_i16_max() {
    // Given: Empty text with i16::MAX shift
    let text = "";
    let shift = i16::MAX;

    // When: Decrypting safely
    let result = decrypt_safe(text, shift);

    // Then: Returns Err(EmptyText) - empty text check comes first
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), CipherError::EmptyText));
}

#[test]
fn test_decrypt_safe_i16_max_error_message_contains_shift() {
    // Given: i16::MAX shift value
    let text = "Test";
    let shift = i16::MAX;

    // When: Decrypting safely
    let result = decrypt_safe(text, shift);

    // Then: Error message contains the shift value
    let error = result.unwrap_err();
    let error_msg = error.to_string();
    assert!(
        error_msg.contains(&i16::MAX.to_string()),
        "Error message should contain shift value {}: got '{}'",
        i16::MAX,
        error_msg
    );
}

#[test]
fn test_decrypt_safe_i16_min_error_message_contains_shift() {
    // Given: i16::MIN shift value
    let text = "Test";
    let shift = i16::MIN;

    // When: Decrypting safely
    let result = decrypt_safe(text, shift);

    // Then: Error message contains the shift value
    let error = result.unwrap_err();
    let error_msg = error.to_string();
    assert!(
        error_msg.contains(&i16::MIN.to_string()),
        "Error message should contain shift value {}: got '{}'",
        i16::MIN,
        error_msg
    );
}

// =============================================================================
// Normal cases (正常系) - Roundtrip verification
// =============================================================================

#[test]
fn test_decrypt_safe_roundtrip_with_encrypt_safe() {
    // Given: Original text and valid shift
    let original = "Hello World";
    let shift = 13;

    // When: Encrypt then decrypt with safe functions
    let encrypted = encrypt_safe(original, shift).unwrap();
    let decrypted = decrypt_safe(&encrypted, shift).unwrap();

    // Then: Returns original text
    assert_eq!(original, decrypted);
}

#[test]
fn test_decrypt_safe_roundtrip_all_valid_shifts() {
    // Given: Test text
    let original = "TestOverflow";

    // When/Then: All valid shifts produce correct roundtrip
    for shift in -25..=25_i16 {
        let encrypted = encrypt_safe(original, shift).unwrap();
        let decrypted = decrypt_safe(&encrypted, shift).unwrap();
        assert_eq!(original, decrypted, "Failed for shift {}", shift);
    }
}
