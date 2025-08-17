# Caesar Cipher Encryption/Decryption

[![Crates.io](https://img.shields.io/crates/v/caesar_cipher_enc_dec.svg)](https://crates.io/crates/caesar_cipher_enc_dec)
[![Documentation](https://docs.rs/caesar_cipher_enc_dec/badge.svg)](https://docs.rs/caesar_cipher_enc_dec)

A comprehensive Caesar cipher implementation in Rust with both library and CLI functionality. Supports uppercase and lowercase letters with robust error handling.

## Features

- 🔐 **Caesar cipher encryption and decryption**
- 🔤 **Both uppercase and lowercase letter support**
- ⚡ **Fast and efficient implementation**
- 🛡️ **Safe functions with error handling**
- 💻 **Command-line interface (CLI)**
- 📁 **File input/output support**
- 🔍 **Brute force decryption**
- 🎯 **Interactive mode**
- ✅ **Comprehensive test coverage**

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
caesar_cipher_enc_dec = "0.6.2"
```

## Library Usage

### Basic Usage

```rust
use caesar_cipher_enc_dec::caesar_cipher::{encrypt, decrypt};

fn main() {
    let text = "Hello World!";
    let encrypted = encrypt(text, 3);
    let decrypted = decrypt(&encrypted, 3);
    
    println!("Original: {}", text);
    println!("Encrypted: {}", encrypted);  // "Khoor Zruog!"
    println!("Decrypted: {}", decrypted);  // "Hello World!"
}
```

### Safe Functions with Error Handling

```rust
use caesar_cipher_enc_dec::caesar_cipher::{encrypt_safe, decrypt_safe};

fn main() {
    match encrypt_safe("Hello World", 3) {
        Ok(encrypted) => println!("Encrypted: {}", encrypted),
        Err(e) => println!("Error: {}", e),
    }
    
    // Error handling for invalid inputs
    match encrypt_safe("", 3) {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {}", e), // "Input text cannot be empty"
    }
    
    match encrypt_safe("Test", 30) {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {}", e), // "Shift value 30 is out of range (-25 to 25)"
    }
}
```

### Brute Force Decryption

```rust
use caesar_cipher_enc_dec::caesar_cipher::decrypt;

fn brute_force_decrypt(encrypted_text: &str) {
    println!("Trying all possible shifts:");
    for shift in 1..=25 {
        let decrypted = decrypt(encrypted_text, shift);
        println!("Shift {}: {}", shift, decrypted);
    }
}
```

## CLI Usage

### Installation

```bash
cargo install caesar_cipher_enc_dec
```

### Basic Commands

```bash
# Encrypt text
caesar_cipher_enc_dec encrypt --text "Hello World" --shift 3

# Decrypt text
caesar_cipher_enc_dec decrypt --text "Khoor Zruog" --shift 3

# Interactive mode
caesar_cipher_enc_dec interactive

# Brute force decryption (try all shifts)
caesar_cipher_enc_dec brute-force --text "Khoor Zruog"

# Show help
caesar_cipher_enc_dec --help
```

### File Operations

```bash
# Encrypt from file
caesar_cipher_enc_dec encrypt --file input.txt --shift 5 --output encrypted.txt

# Decrypt from file
caesar_cipher_enc_dec decrypt --file encrypted.txt --shift 5 --output decrypted.txt
```

### Safe Mode

```bash
# Use safe mode with error checking
caesar_cipher_enc_dec encrypt --text "Hello" --shift 3 --safe
```

## Supported Characters

- **Uppercase letters**: A-Z
- **Lowercase letters**: a-z
- **Other characters**: Numbers, symbols, and non-ASCII characters remain unchanged

## Examples

```rust
use caesar_cipher_enc_dec::caesar_cipher::*;

fn main() {
    // Mixed case with symbols
    let text = "Hello, World! 123";
    let encrypted = encrypt(text, 13);
    println!("{} -> {}", text, encrypted); // "Hello, World! 123" -> "Uryyb, Jbeyq! 123"
    
    // Japanese characters (unchanged)
    let japanese = "こんにちは";
    let result = encrypt(japanese, 5);
    println!("{} -> {}", japanese, result); // "こんにちは" -> "こんにちは"
    
    // Large shift values (automatically normalized)
    let text = "ABC";
    let encrypted = encrypt(text, 27); // Same as shift 1
    println!("{} -> {}", text, encrypted); // "ABC" -> "BCD"
}
```

## Testing

Run the comprehensive test suite:

```bash
cargo test
```

## Error Types

The library includes custom error types for better error handling:

- `CipherError::EmptyText`: When input text is empty
- `CipherError::InvalidShift`: When shift value is outside the valid range (-25 to 25)

## Performance

This implementation is optimized for performance with:
- Zero-allocation character processing
- Efficient modular arithmetic
- Minimal bounds checking

## Links

- [Crates.io](https://crates.io/crates/caesar_cipher_enc_dec)
- [Documentation](https://docs.rs/caesar_cipher_enc_dec)
- [GitHub Repository](https://github.com/username/caesar_cipher_enc_dec)

## License

This project is licensed under the MIT License.
