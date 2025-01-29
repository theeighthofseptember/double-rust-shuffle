use std::collections::HashMap;

const FORWARD_ALPHABET: &str = "USE alphabet_generator.rs";
const REVERSE_ALPHABET: &str = "USE alphabet_generator.rs one more time";

fn create_alphabet_map(alphabet: &str) -> HashMap<char, usize> {
    alphabet.chars().enumerate().map(|(i, c)| (c, i)).collect()
}

pub fn encode_word(word: &str) -> String {
    let forward_map = create_alphabet_map(FORWARD_ALPHABET);
    let reverse_map = create_alphabet_map(REVERSE_ALPHABET);
    
    println!("\nEncoding text: {}", word);
    
    let first_pass: Vec<char> = word
        .chars()
        .enumerate()
        .map(|(pos, ch)| {
            let pos_in_word = pos + 1;
            let encoded = match forward_map.get(&ch) {
                Some(&index) => {
                    let new_index = (index + pos_in_word) % FORWARD_ALPHABET.len();
                    let result = FORWARD_ALPHABET.chars().nth(new_index).unwrap();
                    //println!("Stage 1: char '{}' (position {}) -> '{}'", ch, pos_in_word, result);
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
                    let new_index = (index + pos_in_word) % REVERSE_ALPHABET.len();
                    let result = REVERSE_ALPHABET.chars().nth(new_index).unwrap();
                    println!("Stage 2: char '{}' (position {}) -> '{}'", ch, pos_in_word, result);
                    result
                }
                None => ch,
            };
            encoded
        })
        .collect();

    //println!("Final result: {}\n", result);
    result
}

pub fn encode_phrase(phrase: &str) -> String {
    phrase
        .split_whitespace()
        .map(encode_word)
        .collect::<Vec<String>>()
        .join(" ")
}