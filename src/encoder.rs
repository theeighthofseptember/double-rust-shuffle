use std::collections::HashMap;
use std::sync::OnceLock;

static FORWARD_ALPHABET: OnceLock<String> = OnceLock::new();
static REVERSE_ALPHABET: OnceLock<String> = OnceLock::new();

pub fn set_alphabets(forward: String, reverse: String) {
    let _ = FORWARD_ALPHABET.set(forward);
    let _ = REVERSE_ALPHABET.set(reverse);
}

fn create_alphabet_map(alphabet: &str) -> HashMap<char, usize> {
    alphabet.chars().enumerate().map(|(i, c)| (c, i)).collect()
}

pub fn encode_word(word: &str) -> String {
    let forward_alphabet = FORWARD_ALPHABET.get().expect("Alphabets not initialized");
    let reverse_alphabet = REVERSE_ALPHABET.get().expect("Alphabets not initialized");
    
    println!("\nEncoding text: {}", word);
    
    let forward_map = create_alphabet_map(forward_alphabet);
    let reverse_map = create_alphabet_map(reverse_alphabet);
    
    let first_pass: Vec<char> = word
        .chars()
        .enumerate()
        .map(|(pos, ch)| {
            let pos_in_word = pos + 1;
            let encoded = match forward_map.get(&ch) {
                Some(&index) => {
                    let new_index = (index + pos_in_word) % forward_alphabet.len();
                    let result = forward_alphabet.chars().nth(new_index).unwrap();
                    //println!("Stage 1: char '{}' (position {}) -> '{}'", ch, pos_in_word, result);
                    //dbg!(result);
                    result
                }
                None => ch,
            };
            encoded
        })
        .collect();
    
    //println!("After first stage: {}", first_pass.iter().collect::<String>());

    let result = first_pass
        .iter()
        .enumerate()
        .map(|(pos, &ch)| {
            let pos_in_word = pos + 1;
            let encoded = match reverse_map.get(&ch) {
                Some(&index) => {
                    let new_index = (index + pos_in_word) % reverse_alphabet.len();
                    let result = reverse_alphabet.chars().nth(new_index).unwrap();
                    //println!("Stage 2: char '{}' (position {}) -> '{}'", ch, pos_in_word, result);
                    result
                }
                None => ch,
            };
            encoded
        })
        .collect();

    println!("Final result: {}\n", result);
    result
}

pub fn encode_phrase(phrase: &str) -> String {
    phrase
        .split_whitespace()
        .map(encode_word)
        .collect::<Vec<String>>()
        .join(" ")
}