//! # Caesar Cipher Module
//!
//! Provides easy-to-use Caesar cipher encryption and decryption.
//! Set text and shift number to encrypt or decrypt.
//!
//! # Usage
//!
//! ```
//! use caesar_cipher_enc_dec::caesar_cipher::{encrypt, decrypt, encrypt_safe, decrypt_safe};
//!
//! let text = "I Love You";
//! let enc_text = encrypt(&text, 3);
//! let dec_text = decrypt(&enc_text, 3);
//! let dec_text_2 = encrypt(&enc_text, -3);
//!
//! // Safe versions with error handling
//! match encrypt_safe(&text, 3) {
//!     Ok(encrypted) => println!("Encrypted: {}", encrypted),
//!     Err(e) => println!("Error: {}", e),
//! }
//! ```
//!
//! # Brute Force Example
//!
//! You can use this encrypt code for brute force decryption:
//!
//! ```
//! use caesar_cipher_enc_dec::caesar_cipher::encrypt;
//!
//! let text = "I Love You";
//! for i in 0..26 {
//!     encrypt(&text, i);
//! }
//! ```

/// Size of the alphabet (A-Z)
const ALPHABET_SIZE: i16 = 26;

/// Maximum valid shift value for safe functions
const MAX_SHIFT: i16 = 25;

/// ASCII value of uppercase 'A'
const UPPERCASE_BASE: i16 = 'A' as i16;

/// ASCII value of lowercase 'a'
const LOWERCASE_BASE: i16 = 'a' as i16;

/// Error enum for Caesar cipher operations
///
/// This error type represents possible errors that can occur during encryption/decryption operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CipherError {
    /// Error for invalid shift values
    ///
    /// Occurs when the shift value is outside the range of -25 to 25.
    InvalidShift(String),
    /// Error for empty text input
    EmptyText,
}

impl std::fmt::Display for CipherError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CipherError::InvalidShift(msg) => write!(f, "Invalid shift value: {}", msg),
            CipherError::EmptyText => write!(f, "Input text cannot be empty"),
        }
    }
}

impl std::error::Error for CipherError {}

/// Encrypts text using Caesar cipher
///
/// This function encrypts the specified text with the given shift value.
/// Only alphabetic characters are encrypted, other characters remain unchanged.
///
/// # Arguments
///
/// * `text` - Text to encrypt
/// * `shift` - Shift value (positive for forward shift, negative for backward shift)
///
/// # Returns
///
/// Encrypted text
///
/// # Examples
///
/// ```
/// use caesar_cipher_enc_dec::caesar_cipher::encrypt;
/// 
/// let result = encrypt("Hello", 3);
/// assert_eq!(result, "Khoor");
/// ```
pub fn encrypt(text: &str, shift: i16) -> String {
    encrypt_char(text, shift)
}

/// Decrypts text using Caesar cipher
///
/// This function decrypts the specified text with the given shift value.
/// Internally calls encrypt_char with a negative shift value.
///
/// # Arguments
///
/// * `text` - Text to decrypt
/// * `shift` - Shift value to use for decryption
///
/// # Returns
///
/// Decrypted text
///
/// # Examples
///
/// ```
/// use caesar_cipher_enc_dec::caesar_cipher::decrypt;
/// 
/// let result = decrypt("Khoor", 3);
/// assert_eq!(result, "Hello");
/// ```
pub fn decrypt(text: &str, shift: i16) -> String {
    encrypt_char(text, -shift)
}

/// Encrypts text using Caesar cipher with error handling
///
/// This function validates input values and returns an error for invalid inputs.
/// Returns errors for empty text or out-of-range shift values (outside -25 to 25).
///
/// # Arguments
///
/// * `text` - Text to encrypt (must not be empty)
/// * `shift` - Shift value (must be in range -25 to 25)
///
/// # Returns
///
/// Encrypted text on success, `CipherError` on failure
///
/// # Errors
///
/// * `CipherError::EmptyText` - When text is empty
/// * `CipherError::InvalidShift` - When shift value is out of range
///
/// # Examples
///
/// ```
/// use caesar_cipher_enc_dec::caesar_cipher::encrypt_safe;
/// 
/// let result = encrypt_safe("Hello", 3).unwrap();
/// assert_eq!(result, "Khoor");
/// 
/// // Error cases
/// assert!(encrypt_safe("", 3).is_err());
/// assert!(encrypt_safe("Hello", 26).is_err());
/// ```
pub fn encrypt_safe(text: &str, shift: i16) -> Result<String, CipherError> {
    if text.is_empty() {
        return Err(CipherError::EmptyText);
    }

    if shift.abs() > MAX_SHIFT {
        return Err(CipherError::InvalidShift(format!(
            "Shift value {} is out of range (-{} to {})",
            shift, MAX_SHIFT, MAX_SHIFT
        )));
    }

    Ok(encrypt_char(text, shift))
}

/// Decrypts text using Caesar cipher with error handling
///
/// This function validates input values and returns an error for invalid inputs.
/// Internally calls encrypt_safe with a negative shift value.
///
/// # Arguments
///
/// * `text` - Text to decrypt (must not be empty)
/// * `shift` - Shift value to use for decryption (must be in range -25 to 25)
///
/// # Returns
///
/// Decrypted text on success, `CipherError` on failure
///
/// # Errors
///
/// * `CipherError::EmptyText` - When text is empty
/// * `CipherError::InvalidShift` - When shift value is out of range
///
/// # Examples
///
/// ```
/// use caesar_cipher_enc_dec::caesar_cipher::decrypt_safe;
/// 
/// let result = decrypt_safe("Khoor", 3).unwrap();
/// assert_eq!(result, "Hello");
/// ```
pub fn decrypt_safe(text: &str, shift: i16) -> Result<String, CipherError> {
    encrypt_safe(text, -shift)
}

/// Internal implementation: Performs character-level Caesar cipher transformation
///
/// This function handles the actual encryption processing.
/// Only alphabetic characters (A-Z, a-z) are transformed, other characters remain unchanged.
/// Shift values are automatically normalized to the 0-25 range using Euclidean modulo.
///
/// # Arguments
///
/// * `text` - Text to transform
/// * `shift` - Shift value (automatically normalized via `rem_euclid`)
///
/// # Returns
///
/// Transformed text
fn encrypt_char(text: &str, shift: i16) -> String {
    // Use rem_euclid for proper handling of negative shifts
    let normalized_shift = shift.rem_euclid(ALPHABET_SIZE);

    text.chars()
        .map(|c| match c {
            'A'..='Z' => {
                let shifted = (c as i16 - UPPERCASE_BASE + normalized_shift).rem_euclid(ALPHABET_SIZE);
                ((shifted + UPPERCASE_BASE) as u8) as char
            }
            'a'..='z' => {
                let shifted = (c as i16 - LOWERCASE_BASE + normalized_shift).rem_euclid(ALPHABET_SIZE);
                ((shifted + LOWERCASE_BASE) as u8) as char
            }
            _ => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_uppercase() {
        assert_eq!(encrypt("ABC", 1), "BCD");
        assert_eq!(encrypt("XYZ", 3), "ABC");
        assert_eq!(encrypt("HELLO", 3), "KHOOR");
    }

    #[test]
    fn test_encrypt_lowercase() {
        assert_eq!(encrypt("abc", 1), "bcd");
        assert_eq!(encrypt("xyz", 3), "abc");
        assert_eq!(encrypt("hello", 3), "khoor");
    }

    #[test]
    fn test_encrypt_mixed_case() {
        assert_eq!(encrypt("Hello World", 3), "Khoor Zruog");
        assert_eq!(encrypt("AbC", 1), "BcD");
    }

    #[test]
    fn test_encrypt_with_non_alphabetic() {
        assert_eq!(encrypt("Hello, World! 123", 3), "Khoor, Zruog! 123");
        assert_eq!(encrypt("Test@#$", 5), "Yjxy@#$");
    }

    #[test]
    fn test_decrypt() {
        assert_eq!(decrypt("BCD", 1), "ABC");
        assert_eq!(decrypt("ABC", 3), "XYZ");
        assert_eq!(decrypt("KHOOR", 3), "HELLO");
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let original = "Hello World! 123";
        let shift = 7;
        let encrypted = encrypt(original, shift);
        let decrypted = decrypt(&encrypted, shift);
        assert_eq!(original, decrypted);
    }

    #[test]
    fn test_negative_shift() {
        assert_eq!(encrypt("ABC", -1), "ZAB");
        assert_eq!(encrypt("abc", -1), "zab");
    }

    #[test]
    fn test_large_shift() {
        assert_eq!(encrypt("ABC", 26), "ABC");
        assert_eq!(encrypt("ABC", 27), "BCD");
        assert_eq!(encrypt("ABC", -26), "ABC");
    }

    #[test]
    fn test_encrypt_safe_valid() {
        assert_eq!(encrypt_safe("Hello", 3).unwrap(), "Khoor");
        assert_eq!(encrypt_safe("Test", 25).unwrap(), "Sdrs");
        assert_eq!(encrypt_safe("Test", -25).unwrap(), "Uftu");
    }

    #[test]
    fn test_encrypt_safe_empty_text() {
        assert!(matches!(encrypt_safe("", 3), Err(CipherError::EmptyText)));
    }

    #[test]
    fn test_encrypt_safe_invalid_shift() {
        assert!(matches!(encrypt_safe("Test", 26), Err(CipherError::InvalidShift(_))));
        assert!(matches!(encrypt_safe("Test", -26), Err(CipherError::InvalidShift(_))));
        assert!(matches!(encrypt_safe("Test", 100), Err(CipherError::InvalidShift(_))));
    }

    #[test]
    fn test_decrypt_safe() {
        assert_eq!(decrypt_safe("Khoor", 3).unwrap(), "Hello");
        assert!(matches!(decrypt_safe("", 3), Err(CipherError::EmptyText)));
        assert!(matches!(decrypt_safe("Test", 26), Err(CipherError::InvalidShift(_))));
    }

    #[test]
    fn test_zero_shift() {
        let text = "Hello World";
        assert_eq!(encrypt(text, 0), text);
        assert_eq!(decrypt(text, 0), text);
    }

    #[test]
    fn test_japanese_characters() {
        let text = "こんにちは";
        assert_eq!(encrypt(text, 3), text);
    }

    #[test]
    fn test_special_characters() {
        let text = "!@#$%^&*()";
        assert_eq!(encrypt(text, 5), text);
    }
}