use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Write};
use crate::caesar_cipher::{encrypt, decrypt, encrypt_safe, decrypt_safe};

/// Main CLI structure for the Caesar cipher application
///
/// This structure defines the command-line interface for the Caesar cipher tool,
/// supporting various operations like encryption, decryption, interactive mode, and brute force.
#[derive(Parser)]
#[command(name = "caesar_cipher")]
#[command(about = "A Caesar cipher encryption/decryption tool")]
#[command(version = "0.6.2")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available commands for the Caesar cipher CLI
///
/// Each variant represents a different operation that can be performed
/// with the Caesar cipher tool.
#[derive(Subcommand)]
pub enum Commands {
    /// Encrypt text using Caesar cipher
    Encrypt {
        /// Text to encrypt
        #[arg(short, long)]
        text: Option<String>,
        
        /// Input file path
        #[arg(short = 'f', long)]
        file: Option<String>,
        
        /// Shift value (1-25)
        #[arg(short, long, default_value = "3")]
        shift: i16,
        
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
        
        /// Use safe mode with error checking
        #[arg(long)]
        safe: bool,
    },
    /// Decrypt text using Caesar cipher
    Decrypt {
        /// Text to decrypt
        #[arg(short, long)]
        text: Option<String>,
        
        /// Input file path
        #[arg(short = 'f', long)]
        file: Option<String>,
        
        /// Shift value (1-25)
        #[arg(short, long, default_value = "3")]
        shift: i16,
        
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
        
        /// Use safe mode with error checking
        #[arg(long)]
        safe: bool,
    },
    /// Interactive mode
    Interactive,
    /// Show all possible decryptions (brute force)
    BruteForce {
        /// Text to decrypt
        #[arg(short, long)]
        text: Option<String>,
        
        /// Input file path
        #[arg(short = 'f', long)]
        file: Option<String>,
    },
}

/// Runs the CLI application
///
/// This function parses command-line arguments and executes the appropriate
/// command based on user input. It handles encryption, decryption, interactive mode,
/// and brute force operations.
///
/// # Returns
///
/// Returns `Ok(())` on successful execution, or an error if something goes wrong.
///
/// # Errors
///
/// Returns an error if:
/// - File operations fail
/// - Invalid input is provided to safe functions
/// - Input/output operations fail
pub fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Encrypt { text, file, shift, output, safe } => {
            let input_text = get_input_text(text, file)?;
            let result = if safe {
                encrypt_safe(&input_text, shift)?
            } else {
                encrypt(&input_text, shift)
            };
            output_result(&result, output)?;
        }
        
        Commands::Decrypt { text, file, shift, output, safe } => {
            let input_text = get_input_text(text, file)?;
            let result = if safe {
                decrypt_safe(&input_text, shift)?
            } else {
                decrypt(&input_text, shift)
            };
            output_result(&result, output)?;
        }
        
        Commands::Interactive => {
            run_interactive_mode()?;
        }
        
        Commands::BruteForce { text, file } => {
            let input_text = get_input_text(text, file)?;
            run_brute_force(&input_text);
        }
    }
    
    Ok(())
}

/// Gets input text from either command-line argument, file, or stdin
///
/// This function handles the logic for obtaining input text from various sources.
/// It prioritizes direct text input, then file input, and falls back to stdin if neither is provided.
///
/// # Arguments
///
/// * `text` - Optional text string provided via command line
/// * `file` - Optional file path to read text from
///
/// # Returns
///
/// The input text as a String
///
/// # Errors
///
/// Returns an error if:
/// - Both text and file are provided simultaneously
/// - File reading fails
/// - Stdin reading fails
fn get_input_text(text: Option<String>, file: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    match (text, file) {
        (Some(t), None) => Ok(t),
        (None, Some(f)) => Ok(fs::read_to_string(f)?),
        (Some(_), Some(_)) => Err("Cannot specify both text and file".into()),
        (None, None) => {
            print!("Enter text: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            Ok(input.trim().to_string())
        }
    }
}

/// Outputs the result to either a file or stdout
///
/// This function handles outputting the cipher result to the specified destination.
/// If an output file is provided, writes to that file; otherwise prints to stdout.
///
/// # Arguments
///
/// * `result` - The text result to output
/// * `output_file` - Optional file path to write the result to
///
/// # Returns
///
/// Returns `Ok(())` on success
///
/// # Errors
///
/// Returns an error if file writing fails
fn output_result(result: &str, output_file: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    match output_file {
        Some(file) => {
            fs::write(file, result)?;
            println!("Result written to file");
        }
        None => {
            println!("{}", result);
        }
    }
    Ok(())
}

/// Runs the interactive mode for the Caesar cipher
///
/// This function provides an interactive command-line interface where users
/// can repeatedly perform encryption, decryption, and brute force operations
/// without restarting the program.
///
/// # Returns
///
/// Returns `Ok(())` on successful completion
///
/// # Errors
///
/// Returns an error if input/output operations fail
fn run_interactive_mode() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Caesar Cipher Interactive Mode ===");
    println!("Type 'quit' to exit");
    
    loop {
        print!("\nChoose operation (e)ncrypt, (d)ecrypt, (b)rute force, or (q)uit: ");
        io::stdout().flush()?;
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim().to_lowercase();
        
        match choice.as_str() {
            "e" | "encrypt" => {
                print!("Enter text to encrypt: ");
                io::stdout().flush()?;
                let mut text = String::new();
                io::stdin().read_line(&mut text)?;
                let text = text.trim();
                
                print!("Enter shift value (1-25): ");
                io::stdout().flush()?;
                let mut shift_str = String::new();
                io::stdin().read_line(&mut shift_str)?;
                let shift: i16 = shift_str.trim().parse().unwrap_or(3);
                
                let result = encrypt(text, shift);
                println!("Encrypted: {}", result);
            }
            
            "d" | "decrypt" => {
                print!("Enter text to decrypt: ");
                io::stdout().flush()?;
                let mut text = String::new();
                io::stdin().read_line(&mut text)?;
                let text = text.trim();
                
                print!("Enter shift value (1-25): ");
                io::stdout().flush()?;
                let mut shift_str = String::new();
                io::stdin().read_line(&mut shift_str)?;
                let shift: i16 = shift_str.trim().parse().unwrap_or(3);
                
                let result = decrypt(text, shift);
                println!("Decrypted: {}", result);
            }
            
            "b" | "brute" | "bruteforce" => {
                print!("Enter text to brute force decrypt: ");
                io::stdout().flush()?;
                let mut text = String::new();
                io::stdin().read_line(&mut text)?;
                let text = text.trim();
                
                run_brute_force(text);
            }
            
            "q" | "quit" => {
                println!("Goodbye!");
                break;
            }
            
            _ => {
                println!("Invalid option. Please choose e, d, b, or q.");
            }
        }
    }
    
    Ok(())
}

/// Performs brute force decryption on the given text
///
/// This function attempts to decrypt the input text using all possible
/// shift values (1-25) and displays the results. This is useful when
/// the shift value is unknown.
///
/// # Arguments
///
/// * `text` - The encrypted text to brute force decrypt
fn run_brute_force(text: &str) {
    println!("\n=== Brute Force Decryption ===");
    println!("Original: {}", text);
    println!("Trying all possible shifts:");
    
    for shift in 1..=25 {
        let decrypted = decrypt(text, shift);
        println!("Shift {:2}: {}", shift, decrypted);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_get_input_text_from_string() {
        let result = get_input_text(Some("Hello".to_string()), None).unwrap();
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_get_input_text_from_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Test content").unwrap();
        
        let result = get_input_text(None, Some(temp_file.path().to_string_lossy().to_string())).unwrap();
        assert_eq!(result.trim(), "Test content");
    }

    #[test]
    fn test_get_input_text_both_provided() {
        let result = get_input_text(Some("Hello".to_string()), Some("file.txt".to_string()));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cannot specify both text and file"));
    }

    #[test]
    fn test_output_result_to_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_string_lossy().to_string();
        
        output_result("Test output", Some(file_path.clone())).unwrap();
        
        let content = fs::read_to_string(file_path).unwrap();
        assert_eq!(content, "Test output");
    }
}