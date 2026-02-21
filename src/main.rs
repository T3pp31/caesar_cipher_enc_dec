//! # Caesar Cipher CLI Application
//!
//! This is the main binary for the Caesar cipher encryption/decryption tool.
//! It provides both a command-line interface and a demonstration mode.

use caesar_cipher_enc_dec::caesar_cipher::{decrypt, encrypt, encrypt_safe};
use caesar_cipher_enc_dec::cli;
use std::env;

/// Main entry point for the Caesar cipher application
///
/// This function determines whether to run the CLI interface (when arguments are provided)
/// or the demonstration mode (when no arguments are provided).
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if let Err(e) = cli::run_cli() {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    } else {
        run_demo();
    }
}

/// Runs a demonstration of the Caesar cipher functionality
///
/// This function showcases the various features of the Caesar cipher library,
/// including basic encryption/decryption, mixed case handling, error handling,
/// and displays usage examples for the CLI.
///
/// The demo includes:
/// - Basic encryption and decryption operations
/// - Mixed case text handling
/// - Error handling demonstrations
/// - CLI usage examples
fn run_demo() {
    println!("=== Caesar Cipher Demo ===");
    println!("Run with --help to see CLI options\n");

    // Basic encryption and decryption
    let text = "I Love You.";
    let enc_text = encrypt(text, 3);
    let dec_text = encrypt(&enc_text, -3);
    let dec_text2 = decrypt(&enc_text, 3);

    println!("=== Basic Features ===");
    println!("Original: {}", text);
    println!("Encrypted: {}", enc_text);
    println!("Decrypted (encrypt): {}", dec_text);
    println!("Decrypted (decrypt): {}", dec_text2);

    // Mixed case test
    let mixed_text = "Hello World! 123";
    let encrypted_mixed = encrypt(mixed_text, 5);
    let decrypted_mixed = decrypt(&encrypted_mixed, 5);

    println!("\n=== Mixed Case Test ===");
    println!("Original: {}", mixed_text);
    println!("Encrypted: {}", encrypted_mixed);
    println!("Decrypted: {}", decrypted_mixed);

    // Error handling test
    println!("\n=== Error Handling Test ===");

    match encrypt_safe("Test Message", 3) {
        Ok(result) => println!("Valid encryption: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match encrypt_safe("", 3) {
        Ok(result) => println!("Empty text encryption: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match encrypt_safe("Test", 30) {
        Ok(result) => println!("Invalid shift result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== CLI Usage Examples ===");
    println!("cargo run -- encrypt --text \"Hello World\" --shift 5");
    println!("cargo run -- decrypt --text \"Mjqqt Btwqi\" --shift 5");
    println!("cargo run -- interactive");
    println!("cargo run -- brute-force --text \"Mjqqt Btwqi\"");
    println!("cargo run -- --help");
}
