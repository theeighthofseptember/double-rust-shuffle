# Double Rust Shuffle

A two-stage positional cipher implemented in Rust 🦀

## Overview
Double Rust Shuffle is a positional cipher that performs encryption in two stages, making it more resistant to frequency analysis attacks.

## Features
- Two-stage encryption process
- Position-based shuffling
- Written in pure Rust
- Automatic alphabet generation
- No code editing required

## Installation
1. Download the latest release
2. Run the executable

## Usage
=======
### First Time Setup
1. Run the program
2. Answer 'y' when asked if this is your first time
3. The program will generate two random alphabets
4. Save both alphabets - you'll need them for future encoding/decoding

### Regular Usage
1. Run the program
2. Answer 'n' when asked if this is your first time
3. Enter your saved alphabets when prompted
4. Choose to encode or decode text

### Important
- Always save your alphabets! You need the same alphabets to decode that were used to encode
- Keep your alphabets secret - they serve as your encryption key
- Store alphabets in a secure place

## How it works
>>>>>>> adcdd8a (v2.0.0: Major update - Simplified user experience)

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

## Version History
### v2.0.0
- Complete overhaul of user experience
- Unified program into single executable
- Added automatic alphabet generation
- Removed need for manual code editing
- Runtime alphabet initialization

### v1.0.0
- Initial release
- Required manual alphabet setup
- Separate encoder and decoder executables

## Security Features
- Two-step process increases complexity
- Position-based shifting prevents simple frequency analysis
- Different alphabets for each step prevent reverse-engineering
- Random alphabet generation ensures unique encryption patterns

## Author
[RYCKY]
