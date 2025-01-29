use std::io::{self, Write};

mod encoder;
mod decoder;

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    loop {
        println!("\n=== Encoder/Decoder ===");
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