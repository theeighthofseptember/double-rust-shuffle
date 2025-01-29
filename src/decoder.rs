use std::collections::HashMap;

const FORWARD_ALPHABET: &str = "USE alphabet_generator.rs";
const REVERSE_ALPHABET: &str = "USE alphabet_generator.rs one more time";

fn create_alphabet_map(alphabet: &str) -> HashMap<char, usize> {
    alphabet.chars().enumerate().map(|(i, c)| (c, i)).collect()
}

pub fn decode_word(encoded_word: &str) -> String {
    let forward_map = create_alphabet_map(FORWARD_ALPHABET);
    let reverse_map = create_alphabet_map(REVERSE_ALPHABET);
    
    println!("\nDecoding text: {}", encoded_word);
    
    let second_pass: Vec<char> = encoded_word
        .chars()
        .enumerate()
        .map(|(pos, ch)| {
            let pos_in_word = pos + 1;
            let decoded = match reverse_map.get(&ch) {
                Some(&index) => {
                    let alphabet_len = REVERSE_ALPHABET.len();
                    let new_index = (index as isize - pos_in_word as isize + alphabet_len as isize) 
                        % alphabet_len as isize;
                    let result = REVERSE_ALPHABET.chars().nth(new_index as usize).unwrap();
                    // println!("Stage 1 (reverse second): char '{}' (position {}) -> '{}'", 
                    //    ch, pos_in_word, result);
                    result
                }
                None => ch,
            };
            decoded
        })
        .collect();

    //println!("After first stage: {}", second_pass.iter().collect::<String>());

    // Обратный первый этап (работаем с FORWARD_ALPHABET)
    let result = second_pass
        .iter()
        .enumerate()
        .map(|(pos, &ch)| {
            let pos_in_word = pos + 1;
            let decoded = match forward_map.get(&ch) {
                Some(&index) => {
                    let alphabet_len = FORWARD_ALPHABET.len();
                    let new_index = (index as isize - pos_in_word as isize + alphabet_len as isize) 
                        % alphabet_len as isize;
                    let result = FORWARD_ALPHABET.chars().nth(new_index as usize).unwrap();
                    //println!("Stage 2 (reverse first): char '{}' (position {}) -> '{}'", 
                    //    ch, pos_in_word, result);
                    result
                }
                None => ch,
            };
            decoded
        })
        .collect();

    //println!("Final result: {}\n", result);
    result
}

pub fn decode_phrase(encoded_phrase: &str) -> String {
    encoded_phrase
        .split_whitespace()
        .map(decode_word)
        .collect::<Vec<String>>()
        .join(" ")
}