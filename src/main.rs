use std::path::PathBuf;
use std::fs;

use rand::Rng;
use clap::{Parser};

const CHARACTERS_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const CHARACTERS_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const CHARACTERS_NUMBERS: &str = "1234567890";
const CHARACTERS_SPECIAL: &str = "?/~^{}[]!@#$%&*()_-+=.,:;";

/// Generate a password based on the provided criteria.
#[derive(Parser)]
struct Cli {
    /// Use uppercase characters (optional, default is true)
    #[arg(short, long, default_value_t = false)]
    uppercase: bool,

    /// Use lowercase characters (optional, default is true)
    #[arg(short, long, default_value_t = true)]
    lowercase: bool,

    /// Use numbers characters (optional, default is true)
    #[arg(short, long, default_value_t = false)]
    numbers: bool,

    /// Use special characters (optional, default is true)
    #[arg(short, long, default_value_t = false)]
    special: bool,

    /// Set the length of generated password (default is 8)
    #[arg(short, long, default_value_t = 8)]
    password_length: u32,

    /// Save the password on a file (optional)
    file_output: Option<PathBuf>,
}

fn main() {
    // Get the CLI args
    let args = Cli::parse();
    let mut generated_password: String = String::new();

    // Get a random character for the length of password
    for _ in 0..args.password_length {
        let chars: String = get_characters(args.uppercase, args.lowercase, args.numbers, args.special);

        generated_password.push(get_random_char(chars));
    }

    // Save generated password on a file or print it
    if let Some(path) = args.file_output {
        save_password_to_file(&path, &generated_password);
    } else {
        println!("{}", generated_password);
    }
}

// Get the characters to generate the password
fn get_characters(uppercase: bool, lowercase: bool, numbers: bool, special: bool) -> String {
    let mut chars: String = String::new();

    if uppercase {
        chars.push_str(CHARACTERS_UPPER);
    }

    if lowercase {
        chars.push_str(CHARACTERS_LOWER);
    }

    if numbers {
        chars.push_str(CHARACTERS_NUMBERS);
    }

    if special {
        chars.push_str(CHARACTERS_SPECIAL);
    }

    chars
}

// Get a random character from the provide string
fn get_random_char(characters: String) -> char {
    let mut rng = rand::rng(); // Correctly use `rand::thread_rng()`
    let index = rng.random_range(0..characters.len()); // Correctly use `gen_range`
    characters.chars().nth(index).unwrap()
}

// Save the given password to a file
fn save_password_to_file(path: &PathBuf, password: &str) {
    fs::write(path, password).expect("Invalid path");
}