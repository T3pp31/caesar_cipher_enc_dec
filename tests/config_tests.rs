//! # Config Module Tests
//!
//! Tests for the centralized configuration constants.
//!
//! ## テスト観点表（等価分割・境界値）
//!
//! | テスト観点               | 分類     | 期待値                           |
//! |--------------------------|----------|----------------------------------|
//! | ALPHABET_SIZE値          | 正常系   | 26                               |
//! | MAX_SHIFT値              | 正常系   | 25                               |
//! | UPPERCASE_BASE値         | 正常系   | 65 ('A')                         |
//! | LOWERCASE_BASE値         | 正常系   | 97 ('a')                         |
//! | MAX_BRUTE_FORCE_SHIFT値  | 正常系   | 25                               |
//! | DEFAULT_SHIFT値          | 正常系   | 3                                |
//! | ALPHABET_SIZE == MAX_SHIFT+1 | 境界値 | 関係性が成立                  |
//! | UPPERCASE_BASE < LOWERCASE_BASE | 境界値 | 大文字ベース < 小文字ベース |
//! | MAX_SHIFT < ALPHABET_SIZE | 境界値 | MAX_SHIFTがアルファベットサイズ未満 |
//! | DEFAULT_SHIFT in valid range | 境界値 | 1 <= DEFAULT_SHIFT <= MAX_SHIFT |
//! | MAX_BRUTE_FORCE_SHIFT == MAX_SHIFT | 境界値 | 一致すること |
//! | 各定数が正の値           | 異常系防止 | 全定数 > 0                     |

use caesar_cipher_enc_dec::config::{
    ALPHABET_SIZE, DEFAULT_SHIFT, LOWERCASE_BASE, MAX_BRUTE_FORCE_SHIFT, MAX_SHIFT, UPPERCASE_BASE,
};

// =============================================================================
// Normal cases (正常系) - 各定数の値検証
// =============================================================================

#[test]
fn test_alphabet_size_is_26() {
    // Given: ALPHABET_SIZE constant
    // When: Checking its value
    // Then: Should be 26 (A-Z)
    assert_eq!(ALPHABET_SIZE, 26);
}

#[test]
fn test_max_shift_is_25() {
    // Given: MAX_SHIFT constant
    // When: Checking its value
    // Then: Should be 25 (maximum valid shift)
    assert_eq!(MAX_SHIFT, 25);
}

#[test]
fn test_uppercase_base_is_a() {
    // Given: UPPERCASE_BASE constant
    // When: Checking its value
    // Then: Should be ASCII value of 'A' (65)
    assert_eq!(UPPERCASE_BASE, 'A' as i16);
    assert_eq!(UPPERCASE_BASE, 65);
}

#[test]
fn test_lowercase_base_is_a() {
    // Given: LOWERCASE_BASE constant
    // When: Checking its value
    // Then: Should be ASCII value of 'a' (97)
    assert_eq!(LOWERCASE_BASE, 'a' as i16);
    assert_eq!(LOWERCASE_BASE, 97);
}

#[test]
fn test_max_brute_force_shift_is_25() {
    // Given: MAX_BRUTE_FORCE_SHIFT constant
    // When: Checking its value
    // Then: Should be 25
    assert_eq!(MAX_BRUTE_FORCE_SHIFT, 25);
}

#[test]
fn test_default_shift_is_3() {
    // Given: DEFAULT_SHIFT constant
    // When: Checking its value
    // Then: Should be 3
    assert_eq!(DEFAULT_SHIFT, 3);
}

// =============================================================================
// Boundary values (境界値) - 定数間の関係性検証
// =============================================================================

#[test]
fn test_alphabet_size_equals_max_shift_plus_one() {
    // Given: ALPHABET_SIZE and MAX_SHIFT
    // When: Comparing their relationship
    // Then: ALPHABET_SIZE should equal MAX_SHIFT + 1
    assert_eq!(ALPHABET_SIZE, MAX_SHIFT + 1);
}

#[test]
fn test_uppercase_base_less_than_lowercase_base() {
    // Given: UPPERCASE_BASE and LOWERCASE_BASE
    // When: Comparing them
    // Then: Uppercase base should be less than lowercase base
    assert!(UPPERCASE_BASE < LOWERCASE_BASE);
}

#[test]
fn test_max_shift_less_than_alphabet_size() {
    // Given: MAX_SHIFT and ALPHABET_SIZE
    // When: Comparing them
    // Then: MAX_SHIFT should be strictly less than ALPHABET_SIZE
    assert!(MAX_SHIFT < ALPHABET_SIZE);
}

#[test]
fn test_default_shift_in_valid_range() {
    // Given: DEFAULT_SHIFT and MAX_SHIFT
    // When: Checking range
    // Then: DEFAULT_SHIFT should be between 1 and MAX_SHIFT inclusive
    assert!(DEFAULT_SHIFT >= 1);
    assert!(DEFAULT_SHIFT <= MAX_SHIFT);
}

#[test]
fn test_max_brute_force_shift_equals_max_shift() {
    // Given: MAX_BRUTE_FORCE_SHIFT and MAX_SHIFT
    // When: Comparing them
    // Then: They should be equal
    assert_eq!(MAX_BRUTE_FORCE_SHIFT, MAX_SHIFT);
}

// =============================================================================
// Abnormal prevention (異常系防止) - 全定数が正の値であること
// =============================================================================

#[test]
fn test_all_constants_are_positive() {
    // Given: All configuration constants
    // When: Checking their signs
    // Then: All should be positive
    assert!(ALPHABET_SIZE > 0, "ALPHABET_SIZE must be positive");
    assert!(MAX_SHIFT > 0, "MAX_SHIFT must be positive");
    assert!(UPPERCASE_BASE > 0, "UPPERCASE_BASE must be positive");
    assert!(LOWERCASE_BASE > 0, "LOWERCASE_BASE must be positive");
    assert!(
        MAX_BRUTE_FORCE_SHIFT > 0,
        "MAX_BRUTE_FORCE_SHIFT must be positive"
    );
    assert!(DEFAULT_SHIFT > 0, "DEFAULT_SHIFT must be positive");
}

#[test]
fn test_base_values_are_valid_ascii() {
    // Given: UPPERCASE_BASE and LOWERCASE_BASE
    // When: Checking if they are valid ASCII letter ranges
    // Then: They should be within valid u8 range and represent letters
    assert!(
        UPPERCASE_BASE >= 0 && UPPERCASE_BASE <= 127,
        "UPPERCASE_BASE must be valid ASCII"
    );
    assert!(
        LOWERCASE_BASE >= 0 && LOWERCASE_BASE <= 127,
        "LOWERCASE_BASE must be valid ASCII"
    );
    assert_eq!((UPPERCASE_BASE as u8) as char, 'A');
    assert_eq!((LOWERCASE_BASE as u8) as char, 'a');
}
