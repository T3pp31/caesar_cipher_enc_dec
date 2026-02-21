//! # Safe API Tests
//!
//! Tests for `encrypt_safe` and `decrypt_safe` functions with error handling.

use caesar_cipher_enc_dec::caesar_cipher::{decrypt_safe, encrypt_safe, CipherError};

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
