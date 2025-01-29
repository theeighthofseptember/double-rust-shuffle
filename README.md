# Double Rust Shuffle

A two-step positional cipher implemented in Rust 🦀

## Overview
Double Rust Shuffle is a two-step positional cipher that performs encryption in two stages, making it more resistant to frequency analysis attacks.

## Features
- Two-step encryption process
- Position-based shuffling
- Written in pure Rust
- Custom alphabet support

## Usage
- Run `cargo run --bin alphabet_generator` to generate alphabet for first step of encoding.
![image](https://github.com/user-attachments/assets/6d21fda0-aa72-4b92-a9a1-2535caff0a00)
- Copy the alphabet to the `const FORWARD_ALPHABET: &str = "USE alphabet_generator.rs";` in the `src/encoder.rs` and `src/decoder.rs` files.
- Run `cargo run --bin alphabet_generator` again to generate alphabet for second step of encoding.
![image](https://github.com/user-attachments/assets/90bd9386-426d-414f-8da2-56401a4878fe)
- Copy the alphabet to the `const REVERSE_ALPHABET: &str = "USE alphabet_generator.rs one more time";` in the `src/encoder.rs` and `src/decoder.rs` files.
- Run `cargo run --bin double-rust-shuffle` to encode or decode text.

You can also delete // from the `println!` statements in the `src/encoder.rs` and `src/decoder.rs` files to see how the cipher works.

## Cipher algorithm description

The Double Rust Shuffle cipher implements a two-step positional encryption process:

### Step 1: Forward Shuffle
1. Takes each character of the input text and its position (1-based)
2. Finds the character's index in the first alphabet
3. Shifts this index forward by the character's position in the text
4. Uses modulo operation to wrap around the alphabet length
5. Replaces the character with the one at the new position in the first alphabet

### Step 2: Reverse Shuffle
1. Takes each character from Step 1's output and its position (1-based)
2. Finds the character's index in the second alphabet
3. Shifts this index forward by the character's position in the text
4. Uses modulo operation to wrap around the alphabet length
5. Replaces the character with the one at the new position in the second alphabet

### Key Features
- Uses two different randomly generated alphabets for each step
- Position-dependent shifting makes identical characters encode differently based on their position
- Processes each word separately, preserving word boundaries
- Non-alphabet characters remain unchanged
- Modular arithmetic ensures the result always maps to valid alphabet characters

### Security Features
- Two-step process increases complexity
- Position-based shifting prevents simple frequency analysis
- Different alphabets for each step prevent reverse-engineering
- Random alphabet generation ensures unique encryption patterns

## Author
[RYCKY]
