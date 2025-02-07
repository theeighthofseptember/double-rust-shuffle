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

pub fn decode_word(encoded_word: &str) -> String {
    let forward_alphabet = FORWARD_ALPHABET.get().expect("Alphabets not initialized");
    let reverse_alphabet = REVERSE_ALPHABET.get().expect("Alphabets not initialized");
    
    let forward_map = create_alphabet_map(forward_alphabet);
    let reverse_map = create_alphabet_map(reverse_alphabet);
    
    println!("\nDecoding text: {}", encoded_word);
    
    let second_pass: Vec<char> = encoded_word
        .chars()
        .enumerate()
        .map(|(pos, ch)| {
            let pos_in_word = pos + 1;
            let decoded = match reverse_map.get(&ch) {
                Some(&index) => {
                    let alphabet_len = reverse_alphabet.len();
                    let new_index = (index as isize - pos_in_word as isize + alphabet_len as isize) 
                        % alphabet_len as isize;
                    let result = reverse_alphabet.chars().nth(new_index as usize).unwrap();
                    //println!("Stage 1 (reverse second): char '{}' (position {}) -> '{}'", 
                    //   ch, pos_in_word, result);
                    result
                }
                None => ch,
            };
            decoded
        })
        .collect();

    //println!("After first stage: {}", second_pass.iter().collect::<String>());

    let result = second_pass
        .iter()
        .enumerate()
        .map(|(pos, &ch)| {
            let pos_in_word = pos + 1;
            let decoded = match forward_map.get(&ch) {
                Some(&index) => {
                    let alphabet_len = forward_alphabet.len();
                    let new_index = (index as isize - pos_in_word as isize + alphabet_len as isize) 
                        % alphabet_len as isize;
                    let result = forward_alphabet.chars().nth(new_index as usize).unwrap();
                    //println!("Stage 2 (reverse first): char '{}' (position {}) -> '{}'", 
                    //   ch, pos_in_word, result);
                    result
                }
                None => ch,
            };
            decoded
        })
        .collect();

    println!("Final result: {}\n", result);
    result
}

pub fn decode_phrase(encoded_phrase: &str) -> String {
    encoded_phrase
        .split_whitespace()
        .map(decode_word)
        .collect::<Vec<String>>()
        .join(" ")
}
