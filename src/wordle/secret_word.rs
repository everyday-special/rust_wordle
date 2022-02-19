use std::fs;
use std::collections::HashSet;

use rand::seq::SliceRandom;
use counter::Counter;


pub enum TextColor {
    Green,
    Yellow,
    White,
    Grey,
}

#[derive(Debug)]
pub struct SecretWord {
    word: String,
    allowed_words: HashSet<String>,
}

impl SecretWord {
    pub fn new() -> Self {
        let mut contents = fs::read_to_string("word_list.txt")
            .expect("Something went wrong when reading the secret word file.");
        let secret_words: Vec<String> = contents.split("\n").map(|word| String::from(word)).collect();

        contents = fs::read_to_string("allowed_guesses.txt")
            .expect("Something went wrong when reading the allowed guesses file.");
        let valid_guesses: Vec<String> = contents.split("\n").map(|word| String::from(word)).collect();

        Self {
            word: secret_words.choose(&mut rand::thread_rng()).unwrap().to_string(),
            allowed_words: HashSet::from_iter(valid_guesses.iter().map(|word| word.to_string())),
        }
    }

    pub fn print_word(&self) {
        println!("{}", self.word);
    }

    pub fn word_allowed(&self, guess:&String) -> bool {
        self.allowed_words.contains(guess)
    }

    pub fn check_guess(&self, guess:&String) -> Vec<TextColor> {
        let mut counts = self.word.chars().collect::<Counter<_>>();
        let mut colors: Vec<TextColor> = vec![TextColor::White, TextColor::White, TextColor::White, TextColor::White, TextColor::White];
        for (i, ch) in guess.chars().enumerate() {
            if ch == self.word.chars().nth(i).unwrap() {
                colors[i] = TextColor::Green;
                counts[&ch] -= 1;
                if counts[&ch] == 0 {
                    counts.remove(&ch);
                }
            }
        }
        for (i, ch) in guess.chars().enumerate() {
            if counts.contains_key(&ch) {
                colors[i] = TextColor::Yellow;
                counts[&ch] -= 1;
                if counts[&ch] == 0 {
                    counts.remove(&ch);
                }
            }
        }
        colors
    }
}