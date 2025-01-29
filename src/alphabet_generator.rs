use rand::seq::SliceRandom;
use rand::rngs::ThreadRng;

pub fn generate_random_alphabet() -> String {
    let uppercase: Vec<char> = ('A'..='Z').collect();
    let lowercase: Vec<char> = ('a'..='z').collect();
    let digits: Vec<char> = ('0'..='9').collect();

    let mut all_chars: Vec<char> = Vec::new();
    all_chars.extend(uppercase);
    all_chars.extend(lowercase);
    all_chars.extend(digits);

    let mut rng: ThreadRng = rand::thread_rng();
    all_chars.shuffle(&mut rng);

    all_chars.into_iter().collect()
}

const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

fn main() {
    let alphabet = generate_random_alphabet();
    println!("Generated alphabet: {}\n{RED}Save this if you want to decode your information later.{RESET}"
    , alphabet);
}