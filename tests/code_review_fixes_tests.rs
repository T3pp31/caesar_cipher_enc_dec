//! # Regression Tests for Code Review Fixes
//!
//! This file captures regression tests for three improvements:
//!
//! 1. `decrypt()` no longer overflows when `shift == i16::MIN`
//! 2. CLI default shift matches the `DEFAULT_SHIFT` constant (no hardcoded "3")
//! 3. `encrypt_safe` / `decrypt_safe` docs match behavior for whitespace-only input
//!
//! Each test uses the Given / When / Then comment style.

use caesar_cipher_enc_dec::caesar_cipher::{
    decrypt, decrypt_safe, encrypt, encrypt_safe, CipherError,
};
use caesar_cipher_enc_dec::config::{ALPHABET_SIZE, DEFAULT_SHIFT};

// =============================================================================
// 1. decrypt() integer overflow regression tests
// =============================================================================

#[test]
fn test_decrypt_shift_i16_min_does_not_panic() {
    // Given: shift value equal to i16::MIN (previously caused overflow on -shift)
    let text = "ABC";
    let shift = i16::MIN;

    // When: Decrypting
    let result = decrypt(text, shift);

    // Then: Operation completes without panic and yields a valid rotation
    //       i16::MIN == -32768. -(-32768) mod 26 == 32768 mod 26 == 8.
    //       "ABC" shifted by +8 -> "IJK".
    assert_eq!(result, "IJK");
}

#[test]
fn test_decrypt_shift_i16_min_plus_one() {
    // Given: shift just above the overflow-triggering boundary
    let text = "ABC";
    let shift = i16::MIN + 1;

    // When: Decrypting
    let result = decrypt(text, shift);

    // Then: Matches encrypt with the mathematically equivalent positive shift.
    //       -(-32767) == 32767; 32767 mod 26 == 7.
    let expected = encrypt("ABC", 7);
    assert_eq!(result, expected);
}

#[test]
fn test_decrypt_shift_i16_max() {
    // Given: shift at the opposite signed-integer boundary
    let text = "ABC";
    let shift = i16::MAX;

    // When: Decrypting
    let result = decrypt(text, shift);

    // Then: No panic and result equals encrypt with the equivalent positive shift.
    //       -(32767) mod 26 == (-32767).rem_euclid(26) == 19.
    let expected = encrypt("ABC", 19);
    assert_eq!(result, expected);
}

#[test]
fn test_decrypt_shift_large_negative() {
    // Given: very negative shift well outside the safe range
    let text = "Hello";
    let shift = -30_000;

    // When: Decrypting
    let result = decrypt(text, shift);

    // Then: decrypt(text, s) == encrypt(text, -s) modulo ALPHABET_SIZE,
    //       so decrypt with -30000 must equal encrypt with +30000.
    let expected = encrypt("Hello", 30_000);
    assert_eq!(result, expected);
}

#[test]
fn test_decrypt_shift_alphabet_size_equivalence() {
    // Given: shift equal to ALPHABET_SIZE should be a no-op
    let text = "Hello World";

    // When: Decrypting with ALPHABET_SIZE
    let result = decrypt(text, ALPHABET_SIZE);

    // Then: Text unchanged (full rotation)
    assert_eq!(result, text);
}

#[test]
fn test_decrypt_preserves_encrypt_roundtrip_with_extreme_shift() {
    // Given: encrypt then decrypt with the same large positive shift
    let original = "Hello World";
    let shift = 10_000_i16;

    // When: Round-tripping
    let encrypted = encrypt(original, shift);
    let decrypted = decrypt(&encrypted, shift);

    // Then: Round-trip yields original text
    assert_eq!(decrypted, original);
}

// =============================================================================
// 2. CLI default shift no longer hardcoded
// =============================================================================

#[test]
fn test_cli_default_shift_matches_constant() {
    // Given: clap is configured with `default_value_t = DEFAULT_SHIFT`
    use caesar_cipher_enc_dec::cli::{Cli, Commands};
    use clap::Parser;

    // When: Parsing a command that does not specify --shift
    let cli = Cli::try_parse_from(["caesar_cipher", "encrypt", "--text", "Hello"])
        .expect("parse should succeed");

    // Then: Default value equals DEFAULT_SHIFT constant
    match cli.command {
        Commands::Encrypt(args) => assert_eq!(args.shift, DEFAULT_SHIFT),
        _ => panic!("expected Encrypt subcommand"),
    }
}

#[test]
fn test_cli_default_shift_applies_to_decrypt() {
    // Given: decrypt subcommand without explicit shift
    use caesar_cipher_enc_dec::cli::{Cli, Commands};
    use clap::Parser;

    // When: Parsing
    let cli = Cli::try_parse_from(["caesar_cipher", "decrypt", "--text", "Khoor"])
        .expect("parse should succeed");

    // Then: Default still follows constant, not a hardcoded literal
    match cli.command {
        Commands::Decrypt(args) => assert_eq!(args.shift, DEFAULT_SHIFT),
        _ => panic!("expected Decrypt subcommand"),
    }
}

#[test]
fn test_cli_explicit_shift_overrides_default() {
    // Given: explicit --shift on the command line
    use caesar_cipher_enc_dec::cli::{Cli, Commands};
    use clap::Parser;

    // When: Parsing
    let cli = Cli::try_parse_from(["caesar_cipher", "encrypt", "--text", "Hi", "--shift", "7"])
        .expect("parse should succeed");

    // Then: Explicit value is used, not the default
    match cli.command {
        Commands::Encrypt(args) => assert_eq!(args.shift, 7),
        _ => panic!("expected Encrypt subcommand"),
    }
}

// =============================================================================
// 3. Docstring / behavior alignment: whitespace-only text is rejected
// =============================================================================

#[test]
fn test_encrypt_safe_rejects_empty_string() {
    // Given: empty string input
    let text = "";

    // When: Calling encrypt_safe
    let result = encrypt_safe(text, 3);

    // Then: Returns EmptyText error
    assert!(matches!(result, Err(CipherError::EmptyText)));
}

#[test]
fn test_encrypt_safe_rejects_spaces_only() {
    // Given: whitespace-only (spaces) input
    let text = "     ";

    // When: Calling encrypt_safe
    let result = encrypt_safe(text, 3);

    // Then: Rejected as EmptyText per documented behavior
    assert!(matches!(result, Err(CipherError::EmptyText)));
}

#[test]
fn test_encrypt_safe_rejects_tabs_and_newlines() {
    // Given: whitespace-only (tabs and newlines) input
    let text = "\t\n\r  ";

    // When: Calling encrypt_safe
    let result = encrypt_safe(text, 3);

    // Then: Rejected as EmptyText
    assert!(matches!(result, Err(CipherError::EmptyText)));
}

#[test]
fn test_decrypt_safe_rejects_whitespace_only() {
    // Given: whitespace-only input to decrypt_safe
    let text = "   \t";

    // When: Calling decrypt_safe
    let result = decrypt_safe(text, 3);

    // Then: Rejected consistently with encrypt_safe
    assert!(matches!(result, Err(CipherError::EmptyText)));
}

#[test]
fn test_encrypt_safe_accepts_text_with_surrounding_whitespace() {
    // Given: valid text surrounded by whitespace (not whitespace-only)
    let text = "  Hi  ";

    // When: Calling encrypt_safe
    let result = encrypt_safe(text, 3);

    // Then: Succeeds and whitespace is preserved in output
    let encrypted = result.expect("non-empty text should succeed");
    assert_eq!(encrypted, "  Kl  ");
}

#[test]
fn test_empty_text_error_message_mentions_whitespace() {
    // Given: the CipherError::EmptyText variant
    let err = CipherError::EmptyText;

    // When: Formatting as a string
    let msg = format!("{}", err);

    // Then: Message explicitly mentions whitespace-only rejection so user-facing
    //       messages match documentation
    assert!(
        msg.contains("whitespace"),
        "expected message to mention whitespace, got: {}",
        msg
    );
}

// Execution & coverage:
//   cargo test --test code_review_fixes_tests
//   cargo tarpaulin --tests   (for branch/line coverage, if tarpaulin is installed)
