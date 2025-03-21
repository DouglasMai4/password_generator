# Password Generator

This is a simple command-line password generator written in Rust. It allows you to generate secure passwords with customizable options, such as the inclusion of uppercase letters, lowercase letters, numbers, and special characters. You can also specify the length of the password and save it to a file.

## Features:

- Generate passwords with customizable character sets (uppercase, lowercase, numbers, special characters).
- Specify the length of the generated password.
- Optionally save the generated password to a file.
- Written in **Rust** for fast execution.

## Requirements:

- [Rust Lang](https://www.rust-lang.org)

## Installation

1. Clone the repository

```bash
git clone https://github.com/DouglasMai4/password-generator.git
cd password-generator
```

2. Build the project

```bash
cargo build --release
```

This will compile the project and create an optimized executable.
Usage

You can use the password generator with the following command-line options:
Generate a password

```bash
./target/release/password-generator
```

This will generate a random password with the default settings (8 characters, lowercase letters only).
Custom options

You can customize the generated password by using the following flags:

    -l, --lowercase (default: true) - Include lowercase characters.
    -u, --uppercase (default: false) - Include uppercase characters.
    -n, --numbers (default: false) - Include numbers.
    -s, --special (default: false) - Include special characters.
    -p, --password-length (default: 8) - Set the length of the generated password.
    -f, --file-output (optional) - Save the password to a file.

## Example commands

### Generate a password with lowercase and uppercase letters:

```bash
./target/release/password-generator --uppercase --lowercase
```

### Generate a password with lowercase letters, numbers, and special characters:

```bash
./target/release/password-generator --lowercase --numbers --special
```

### Generate a password of length 12:

```bash
./target/release/password-generator --password-length 12
```

### Save the password to a file:

```bash
./target/release/password-generator --file-output ./password.txt
```

This will save the generated password to the file password.txt in the current directory.
License

This project is licensed under the MIT License.