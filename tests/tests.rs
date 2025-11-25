//! # Integration Tests for Caesar Cipher
//!
//! This module contains comprehensive integration tests based on equivalence partitioning
//! and boundary value analysis as specified in CLAUDE.md.
//!
//! ## Test Categories
//! - Normal cases (正常系)
//! - Abnormal cases (異常系)
//! - Boundary values (境界値)
//!
//! ## Execution
//! ```bash
//! cargo test
//! ```
//!
//! ## Coverage
//! ```bash
//! cargo install cargo-tarpaulin
//! cargo tarpaulin --out Html
//! ```

use caesar_cipher_enc_dec::caesar_cipher::{
    decrypt, decrypt_safe, encrypt, encrypt_safe, CipherError,
};

// =============================================================================
// encrypt function tests
// =============================================================================

mod encrypt_tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Normal cases (正常系)
    // -------------------------------------------------------------------------

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

    // -------------------------------------------------------------------------
    // Boundary values (境界値)
    // -------------------------------------------------------------------------

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

    // -------------------------------------------------------------------------
    // Abnormal cases (異常系)
    // -------------------------------------------------------------------------

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
}

// =============================================================================
// decrypt function tests
// =============================================================================

mod decrypt_tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Normal cases (正常系)
    // -------------------------------------------------------------------------

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

    // -------------------------------------------------------------------------
    // Boundary values (境界値)
    // -------------------------------------------------------------------------

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

    // -------------------------------------------------------------------------
    // Abnormal cases (異常系)
    // -------------------------------------------------------------------------

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
}

// =============================================================================
// Roundtrip tests (encrypt then decrypt)
// =============================================================================

mod roundtrip_tests {
    use super::*;

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
}

// =============================================================================
// encrypt_safe function tests
// =============================================================================

mod encrypt_safe_tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Normal cases (正常系)
    // -------------------------------------------------------------------------

    #[test]
    fn test_encrypt_safe_valid_input() {
        // Given: Valid text and shift
        let text = "Hello";
        let shift = 3;

        // When: Encrypting safely
        let result = encrypt_safe(text, shift);

        // Then: Returns Ok with encrypted text
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Khoor");
    }

    #[test]
    fn test_encrypt_safe_single_character() {
        // Given: Single character
        let text = "A";
        let shift = 1;

        // When: Encrypting safely
        let result = encrypt_safe(text, shift);

        // Then: Returns Ok
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "B");
    }

    // -------------------------------------------------------------------------
    // Boundary values (境界値)
    // -------------------------------------------------------------------------

    #[test]
    fn test_encrypt_safe_shift_zero() {
        // Given: Shift of 0
        let text = "Test";
        let shift = 0;

        // When: Encrypting safely
        let result = encrypt_safe(text, shift);

        // Then: Returns Ok with unchanged text
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Test");
    }

    #[test]
    fn test_encrypt_safe_shift_25_maximum_valid() {
        // Given: Maximum valid shift (25)
        let text = "A";
        let shift = 25;

        // When: Encrypting safely
        let result = encrypt_safe(text, shift);

        // Then: Returns Ok
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Z");
    }

    #[test]
    fn test_encrypt_safe_shift_negative_25_minimum_valid() {
        // Given: Minimum valid shift (-25)
        let text = "Z";
        let shift = -25;

        // When: Encrypting safely
        let result = encrypt_safe(text, shift);

        // Then: Returns Ok
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "A");
    }

    #[test]
    fn test_encrypt_safe_shift_26_boundary_invalid() {
        // Given: Shift of 26 (just outside valid range)
        let text = "Test";
        let shift = 26;

        // When: Encrypting safely
        let result = encrypt_safe(text, shift);

        // Then: Returns Err(InvalidShift)
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CipherError::InvalidShift(_)));
    }

    #[test]
    fn test_encrypt_safe_shift_negative_26_boundary_invalid() {
        // Given: Shift of -26 (just outside valid range)
        let text = "Test";
        let shift = -26;

        // When: Encrypting safely
        let result = encrypt_safe(text, shift);

        // Then: Returns Err(InvalidShift)
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CipherError::InvalidShift(_)));
    }

    // -------------------------------------------------------------------------
    // Abnormal cases (異常系)
    // -------------------------------------------------------------------------

    #[test]
    fn test_encrypt_safe_empty_text() {
        // Given: Empty text
        let text = "";
        let shift = 3;

        // When: Encrypting safely
        let result = encrypt_safe(text, shift);

        // Then: Returns Err(EmptyText)
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CipherError::EmptyText));
    }

    #[test]
    fn test_encrypt_safe_shift_100_far_out_of_range() {
        // Given: Shift far out of range
        let text = "Test";
        let shift = 100;

        // When: Encrypting safely
        let result = encrypt_safe(text, shift);

        // Then: Returns Err(InvalidShift)
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CipherError::InvalidShift(_)));
    }

    #[test]
    fn test_encrypt_safe_shift_negative_100_far_out_of_range() {
        // Given: Negative shift far out of range
        let text = "Test";
        let shift = -100;

        // When: Encrypting safely
        let result = encrypt_safe(text, shift);

        // Then: Returns Err(InvalidShift)
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CipherError::InvalidShift(_)));
    }

    #[test]
    fn test_encrypt_safe_error_message_contains_shift_value() {
        // Given: Invalid shift value
        let text = "Test";
        let shift = 30;

        // When: Encrypting safely
        let result = encrypt_safe(text, shift);

        // Then: Error message contains the shift value
        assert!(result.is_err());
        let error = result.unwrap_err();
        let error_message = error.to_string();
        assert!(
            error_message.contains("30"),
            "Error message should contain shift value 30: {}",
            error_message
        );
    }

    #[test]
    fn test_encrypt_safe_empty_text_error_message() {
        // Given: Empty text
        let text = "";
        let shift = 3;

        // When: Encrypting safely
        let result = encrypt_safe(text, shift);

        // Then: Error message is correct
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.to_string(), "Input text cannot be empty");
    }
}

// =============================================================================
// decrypt_safe function tests
// =============================================================================

mod decrypt_safe_tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Normal cases (正常系)
    // -------------------------------------------------------------------------

    #[test]
    fn test_decrypt_safe_valid_input() {
        // Given: Valid encrypted text and shift
        let text = "Khoor";
        let shift = 3;

        // When: Decrypting safely
        let result = decrypt_safe(text, shift);

        // Then: Returns Ok with decrypted text
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello");
    }

    // -------------------------------------------------------------------------
    // Boundary values (境界値)
    // -------------------------------------------------------------------------

    #[test]
    fn test_decrypt_safe_shift_zero() {
        // Given: Shift of 0
        let text = "Test";
        let shift = 0;

        // When: Decrypting safely
        let result = decrypt_safe(text, shift);

        // Then: Returns Ok with unchanged text
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Test");
    }

    #[test]
    fn test_decrypt_safe_shift_25() {
        // Given: Shift of 25
        let text = "Z";
        let shift = 25;

        // When: Decrypting safely
        let result = decrypt_safe(text, shift);

        // Then: Returns Ok
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "A");
    }

    #[test]
    fn test_decrypt_safe_shift_26_invalid() {
        // Given: Shift of 26 (invalid)
        let text = "Test";
        let shift = 26;

        // When: Decrypting safely
        let result = decrypt_safe(text, shift);

        // Then: Returns Err(InvalidShift)
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CipherError::InvalidShift(_)));
    }

    // -------------------------------------------------------------------------
    // Abnormal cases (異常系)
    // -------------------------------------------------------------------------

    #[test]
    fn test_decrypt_safe_empty_text() {
        // Given: Empty text
        let text = "";
        let shift = 3;

        // When: Decrypting safely
        let result = decrypt_safe(text, shift);

        // Then: Returns Err(EmptyText)
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CipherError::EmptyText));
    }
}

// =============================================================================
// CipherError tests
// =============================================================================

mod cipher_error_tests {
    use super::*;

    #[test]
    fn test_cipher_error_display_empty_text() {
        // Given: EmptyText error
        let error = CipherError::EmptyText;

        // When: Converting to string
        let message = error.to_string();

        // Then: Correct message
        assert_eq!(message, "Input text cannot be empty");
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
}

// =============================================================================
// Safe roundtrip tests
// =============================================================================

mod safe_roundtrip_tests {
    use super::*;

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
}
