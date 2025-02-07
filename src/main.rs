use std::io::{self, Write};

mod encoder;
mod decoder;
mod alphabet_generator;

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn first_time_setup() {
    println!("\n=== Generating alphabets ===");
    
    let forward_alphabet = alphabet_generator::generate_random_alphabet();
    let reverse_alphabet = alphabet_generator::generate_random_alphabet();
    
    println!("\n⚠️ Important: Save these alphabets! You'll need them to decode your messages.");
    println!("First alphabet (for forward shuffle): {}", forward_alphabet);
    println!("Second alphabet (for reverse shuffle): {}", reverse_alphabet);
    
    // Set alphabets in both encoder and decoder
    encoder::set_alphabets(forward_alphabet.clone(), reverse_alphabet.clone());
    decoder::set_alphabets(forward_alphabet, reverse_alphabet);
    
    println!("\nPress Enter to continue to the main menu...");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}

fn main() {
    let first_time = read_input("Is this your first time running the program? (y/n): ")
        .to_lowercase()
        .starts_with('y');
    
    if first_time {
        first_time_setup();
    } else {
        println!("Please enter your saved alphabets:");
        let forward = read_input("Enter first alphabet: ");
        let reverse = read_input("Enter second alphabet: ");
        encoder::set_alphabets(forward.clone(), reverse.clone());
        decoder::set_alphabets(forward, reverse);
    }
    
    loop {
        println!("\n=== Double Rust Shuffle ===");
        println!("1. Encode text");
        println!("2. Decode text");
        println!("3. Exit");
        
        let choice = read_input("Choose (1-3): ");
        
        match choice.as_str() {
            "1" => {
                let text = read_input("Enter text to encode: ");
                let encoded = encoder::encode_phrase(&text);
                println!("Encoded: {}", encoded);
            },
            "2" => {
                let text = read_input("Enter text to decode: ");
                let decoded = decoder::decode_phrase(&text);
                println!("Decoded: {}", decoded);
            },
            "3" => {
                println!("Goodbye!");
                break;
            },
            _ => println!("Invalid choice. Please choose 1, 2 or 3."),
        }
    }
}