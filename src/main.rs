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
    
    // 基本的な暗号化と復号化
    let text: &str = "I Love You.";
    let enc_text: String = encrypt(&text, 3);
    let dec_text: String = encrypt(&enc_text, -3);
    let dec_text2: String = decrypt(&enc_text, 3);
    
    println!("=== 基本機能 ===");
    println!("元の文字列: {}", text);
    println!("暗号化: {}", enc_text);
    println!("復号化(encrypt): {}", dec_text);
    println!("復号化(decrypt): {}", dec_text2);
    
    // 小文字と大文字のテスト
    let mixed_text = "Hello World! 123";
    let encrypted_mixed = encrypt(&mixed_text, 5);
    let decrypted_mixed = decrypt(&encrypted_mixed, 5);
    
    println!("\n=== 大文字小文字混在テスト ===");
    println!("元の文字列: {}", mixed_text);
    println!("暗号化: {}", encrypted_mixed);
    println!("復号化: {}", decrypted_mixed);
    
    // エラーハンドリングのテスト
    println!("\n=== エラーハンドリングテスト ===");
    
    match encrypt_safe("Test Message", 3) {
        Ok(result) => println!("正常な暗号化: {}", result),
        Err(e) => println!("エラー: {}", e),
    }
    
    match encrypt_safe("", 3) {
        Ok(result) => println!("空文字の暗号化: {}", result),
        Err(e) => println!("エラー: {}", e),
    }
    
    match encrypt_safe("Test", 30) {
        Ok(result) => println!("無効なシフト値: {}", result),
        Err(e) => println!("エラー: {}", e),
    }
    
    println!("\n=== CLI使用例 ===");
    println!("cargo run -- encrypt --text \"Hello World\" --shift 5");
    println!("cargo run -- decrypt --text \"Mjqqt Btwqi\" --shift 5");
    println!("cargo run -- interactive");
    println!("cargo run -- brute-force --text \"Mjqqt Btwqi\"");
    println!("cargo run -- --help");
}
