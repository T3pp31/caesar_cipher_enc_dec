use clap::{Args, Parser, Subcommand};
use std::fs;
use std::io::{self, Write};

use crate::caesar_cipher::{decrypt, decrypt_safe, encrypt, encrypt_safe};
use crate::config::{DEFAULT_SHIFT, MAX_BRUTE_FORCE_SHIFT, MAX_INPUT_SIZE, MAX_SHIFT, MIN_SHIFT};

/// Main CLI structure for the Caesar cipher application
///
/// This structure defines the command-line interface for the Caesar cipher tool,
/// supporting various operations like encryption, decryption, interactive mode, and brute force.
#[derive(Parser)]
#[command(name = "caesar_cipher")]
#[command(about = "A Caesar cipher encryption/decryption tool")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Common arguments shared between encrypt and decrypt commands
#[derive(Args)]
pub struct CipherArgs {
    /// Text to process
    #[arg(short, long)]
    pub text: Option<String>,

    /// Input file path
    #[arg(short = 'f', long)]
    pub file: Option<String>,

    /// Shift value (any integer; safe mode: -25 to 25, default: 3)
    #[arg(short, long, default_value = "3")]
    pub shift: i16,

    /// Output file path
    #[arg(short, long)]
    pub output: Option<String>,

    /// Use safe mode with error checking
    #[arg(long)]
    pub safe: bool,
}

/// Available commands for the Caesar cipher CLI
///
/// Each variant represents a different operation that can be performed
/// with the Caesar cipher tool.
#[derive(Subcommand)]
pub enum Commands {
    /// Encrypt text using Caesar cipher
    Encrypt(CipherArgs),
    /// Decrypt text using Caesar cipher
    Decrypt(CipherArgs),
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
        Commands::Encrypt(args) => {
            let input_text = get_input_text(args.text, args.file)?;
            let result = if args.safe {
                encrypt_safe(&input_text, args.shift)?
            } else {
                encrypt(&input_text, args.shift)
            };
            output_result(&result, args.output)?;
        }

        Commands::Decrypt(args) => {
            let input_text = get_input_text(args.text, args.file)?;
            let result = if args.safe {
                decrypt_safe(&input_text, args.shift)?
            } else {
                decrypt(&input_text, args.shift)
            };
            output_result(&result, args.output)?;
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
fn get_input_text(
    text: Option<String>,
    file: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    match (text, file) {
        (Some(t), None) => {
            if t.len() > MAX_INPUT_SIZE {
                return Err(format!(
                    "Input text exceeds maximum size of {} bytes",
                    MAX_INPUT_SIZE
                )
                .into());
            }
            Ok(t)
        }
        (None, Some(f)) => {
            let metadata =
                fs::metadata(&f).map_err(|e| format!("Failed to read file '{}': {}", f, e))?;
            if metadata.len() > MAX_INPUT_SIZE as u64 {
                return Err(format!(
                    "Input file '{}' exceeds maximum size of {} bytes",
                    f, MAX_INPUT_SIZE
                )
                .into());
            }
            fs::read_to_string(&f).map_err(|e| format!("Failed to read file '{}': {}", f, e).into())
        }
        (Some(_), Some(_)) => Err("Cannot specify both text and file".into()),
        (None, None) => {
            print!("Enter text: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if input.len() > MAX_INPUT_SIZE {
                return Err(format!(
                    "Input text exceeds maximum size of {} bytes",
                    MAX_INPUT_SIZE
                )
                .into());
            }
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
fn output_result(
    result: &str,
    output_file: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    match output_file {
        Some(file) => {
            fs::write(&file, result)?;
            println!("Result written to file: {}", file);
        }
        None => {
            println!("{}", result);
        }
    }
    Ok(())
}

/// Prompts the user for text input
///
/// # Arguments
///
/// * `prompt` - The prompt message to display
///
/// # Returns
///
/// The trimmed input string
fn prompt_for_text(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

/// Validates shift input string and returns parsed value with optional warning
///
/// # Arguments
///
/// * `input` - Raw input string from user
///
/// # Returns
///
/// A tuple of (shift value, optional warning message).
/// Returns default shift with warning if input is invalid.
pub(crate) fn validate_shift_input(input: &str) -> (i16, Option<String>) {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return (DEFAULT_SHIFT, None);
    }

    match trimmed.parse::<i16>() {
        Ok(shift) => {
            if !(MIN_SHIFT..=MAX_SHIFT).contains(&shift) {
                let warning = format!(
                    "Warning: shift {} is outside the typical range ({} to {}). Value will be normalized.",
                    shift, MIN_SHIFT, MAX_SHIFT
                );
                (shift, Some(warning))
            } else {
                (shift, None)
            }
        }
        Err(_) => {
            let warning = format!("Invalid shift value, using default ({})", DEFAULT_SHIFT);
            (DEFAULT_SHIFT, Some(warning))
        }
    }
}

/// Prompts the user for a shift value with validation
///
/// # Returns
///
/// A valid shift value, or the default if input is invalid
fn prompt_for_shift() -> io::Result<i16> {
    print!("Enter shift value (default: {}): ", DEFAULT_SHIFT);
    io::stdout().flush()?;
    let mut shift_str = String::new();
    io::stdin().read_line(&mut shift_str)?;

    let (shift, warning) = validate_shift_input(&shift_str);
    if let Some(msg) = warning {
        println!("{}", msg);
    }
    Ok(shift)
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
        let choice =
            prompt_for_text("\nChoose operation (e)ncrypt, (d)ecrypt, (b)rute force, or (q)uit: ")?;
        let choice = choice.to_lowercase();

        match choice.as_str() {
            "e" | "encrypt" => {
                let text = prompt_for_text("Enter text to encrypt: ")?;
                let shift = prompt_for_shift()?;
                let result = encrypt(&text, shift);
                println!("Encrypted: {}", result);
            }

            "d" | "decrypt" => {
                let text = prompt_for_text("Enter text to decrypt: ")?;
                let shift = prompt_for_shift()?;
                let result = decrypt(&text, shift);
                println!("Decrypted: {}", result);
            }

            "b" | "brute" | "bruteforce" => {
                let text = prompt_for_text("Enter text to brute force decrypt: ")?;
                run_brute_force(&text);
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
/// shift values (0-25) and displays the results. This is useful when
/// the shift value is unknown. Shift 0 shows the original text for reference.
///
/// # Arguments
///
/// * `text` - The encrypted text to brute force decrypt
fn run_brute_force(text: &str) {
    println!("\n=== Brute Force Decryption ===");
    println!("Original: {}", text);
    println!("Trying all possible shifts:");

    for shift in 0..=MAX_BRUTE_FORCE_SHIFT {
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

        let result =
            get_input_text(None, Some(temp_file.path().to_string_lossy().to_string())).unwrap();
        assert_eq!(result.trim(), "Test content");
    }

    #[test]
    fn test_get_input_text_both_provided() {
        let result = get_input_text(Some("Hello".to_string()), Some("file.txt".to_string()));
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Cannot specify both text and file"));
    }

    #[test]
    fn test_output_result_to_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_string_lossy().to_string();

        output_result("Test output", Some(file_path.clone())).unwrap();

        let content = fs::read_to_string(file_path).unwrap();
        assert_eq!(content, "Test output");
    }

    // -------------------------------------------------------------------------
    // Input size limit tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_get_input_text_oversized_file_error() {
        // Given: A file that exceeds MAX_INPUT_SIZE
        use std::io::Write as _;
        let mut temp_file = NamedTempFile::new().unwrap();
        // Write MAX_INPUT_SIZE + 1 bytes to exceed the limit
        let oversized_data = vec![b'A'; MAX_INPUT_SIZE + 1];
        temp_file.write_all(&oversized_data).unwrap();
        temp_file.flush().unwrap();

        // When: Reading from oversized file
        let result = get_input_text(None, Some(temp_file.path().to_string_lossy().to_string()));

        // Then: Returns error about exceeding maximum size
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("exceeds maximum size"));
    }

    #[test]
    fn test_get_input_text_file_at_max_size_succeeds() {
        // Given: A file exactly at MAX_INPUT_SIZE (should succeed)
        use std::io::Write as _;
        let mut temp_file = NamedTempFile::new().unwrap();
        let data = vec![b'A'; MAX_INPUT_SIZE];
        temp_file.write_all(&data).unwrap();
        temp_file.flush().unwrap();

        // When: Reading from file at the limit
        let result = get_input_text(None, Some(temp_file.path().to_string_lossy().to_string()));

        // Then: Returns Ok
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), MAX_INPUT_SIZE);
    }

    // -------------------------------------------------------------------------
    // validate_shift_input tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_validate_shift_input_valid_value() {
        // Given: A valid shift value within range
        // When: Validating input "3"
        let (shift, warning) = validate_shift_input("3");
        // Then: Returns 3 with no warning
        assert_eq!(shift, 3);
        assert!(warning.is_none());
    }

    #[test]
    fn test_validate_shift_input_zero() {
        // Given: Shift value of 0 (boundary)
        // When: Validating input "0"
        let (shift, warning) = validate_shift_input("0");
        // Then: Returns 0 with no warning
        assert_eq!(shift, 0);
        assert!(warning.is_none());
    }

    #[test]
    fn test_validate_shift_input_max_boundary() {
        // Given: Maximum valid shift value (25)
        // When: Validating input "25"
        let (shift, warning) = validate_shift_input("25");
        // Then: Returns 25 with no warning
        assert_eq!(shift, 25);
        assert!(warning.is_none());
    }

    #[test]
    fn test_validate_shift_input_min_boundary() {
        // Given: Minimum valid shift value (-25)
        // When: Validating input "-25"
        let (shift, warning) = validate_shift_input("-25");
        // Then: Returns -25 with no warning
        assert_eq!(shift, -25);
        assert!(warning.is_none());
    }

    #[test]
    fn test_validate_shift_input_out_of_range_positive() {
        // Given: Shift value above range (26)
        // When: Validating input "26"
        let (shift, warning) = validate_shift_input("26");
        // Then: Returns 26 with a warning
        assert_eq!(shift, 26);
        assert!(warning.is_some());
        assert!(warning.unwrap().contains("Warning"));
    }

    #[test]
    fn test_validate_shift_input_out_of_range_negative() {
        // Given: Shift value below range (-26)
        // When: Validating input "-26"
        let (shift, warning) = validate_shift_input("-26");
        // Then: Returns -26 with a warning
        assert_eq!(shift, -26);
        assert!(warning.is_some());
        assert!(warning.unwrap().contains("Warning"));
    }

    #[test]
    fn test_validate_shift_input_far_out_of_range() {
        // Given: Shift value far out of range (9999)
        // When: Validating input "9999"
        let (shift, warning) = validate_shift_input("9999");
        // Then: Returns 9999 with a warning containing the value
        assert_eq!(shift, 9999);
        assert!(warning.is_some());
        assert!(warning.unwrap().contains("9999"));
    }

    #[test]
    fn test_validate_shift_input_invalid_string() {
        // Given: Non-numeric input
        // When: Validating input "abc"
        let (shift, warning) = validate_shift_input("abc");
        // Then: Returns default shift with a warning
        assert_eq!(shift, DEFAULT_SHIFT);
        assert!(warning.is_some());
        assert!(warning.unwrap().contains("Invalid"));
    }

    #[test]
    fn test_validate_shift_input_empty_string() {
        // Given: Empty input
        // When: Validating input ""
        let (shift, warning) = validate_shift_input("");
        // Then: Returns default shift with no warning
        assert_eq!(shift, DEFAULT_SHIFT);
        assert!(warning.is_none());
    }

    #[test]
    fn test_validate_shift_input_whitespace_only() {
        // Given: Whitespace-only input
        // When: Validating input "  "
        let (shift, warning) = validate_shift_input("  ");
        // Then: Returns default shift with no warning (treated as empty)
        assert_eq!(shift, DEFAULT_SHIFT);
        assert!(warning.is_none());
    }

    #[test]
    fn test_validate_shift_input_with_surrounding_whitespace() {
        // Given: Valid value with surrounding whitespace
        // When: Validating input " 5 "
        let (shift, warning) = validate_shift_input(" 5 ");
        // Then: Returns 5 with no warning
        assert_eq!(shift, 5);
        assert!(warning.is_none());
    }
}
