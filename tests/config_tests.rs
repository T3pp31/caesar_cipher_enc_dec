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

// Relationships among UPPERCASE_BASE, MAX_SHIFT, DEFAULT_SHIFT, etc. are
// checked at compile time in `src/config.rs` (`const _` block).

#[test]
fn test_max_brute_force_shift_equals_max_shift() {
    // Given: MAX_BRUTE_FORCE_SHIFT and MAX_SHIFT
    // When: Comparing them
    // Then: They should be equal
    assert_eq!(MAX_BRUTE_FORCE_SHIFT, MAX_SHIFT);
}

#[test]
fn test_base_values_are_valid_ascii_letters() {
    // Given: UPPERCASE_BASE and LOWERCASE_BASE (sign/range checked at compile time in config.rs)
    // When: Converting to characters
    // Then: They represent 'A' and 'a'
    assert_eq!((UPPERCASE_BASE as u8) as char, 'A');
    assert_eq!((LOWERCASE_BASE as u8) as char, 'a');
}
