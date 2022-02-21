use std::collections::HashSet;

use counter::Counter;
use rand::seq::SliceRandom;

#[derive(Debug, PartialEq, Eq)]
pub enum TextColor {
    Green,
    Yellow,
    White,
    Black,
}

#[derive(Debug)]
pub struct SecretWord {
    word: String,
    allowed_words: HashSet<String>,
}

impl SecretWord {
    pub fn new() -> Self {
        let contents = include_str!("../../word_list.txt");
        let secret_words: Vec<String> = contents.split("\n").map(String::from).collect();
        let allowed_words = include_str!("../../allowed_guesses.txt")
            .split("\n")
            .map(String::from)
            .collect::<HashSet<String>>();

        Self {
            word: secret_words
                .choose(&mut rand::thread_rng())
                .unwrap()
                .to_string(),
            allowed_words,
        }
    }

    pub fn print_word(&self) {
        println!("{}", self.word);
    }

    pub fn word_allowed(&self, guess: &String) -> bool {
        self.allowed_words.contains(guess)
    }

    pub fn is_exact_match(&self, guess: &String) -> bool {
        return self.word == *guess;
    }

    pub fn check_guess(&self, guess:&String) -> Vec<TextColor> {
        let mut counts = self.word.chars().collect::<Counter<_>>();
        let mut colors: Vec<TextColor> = vec![
            TextColor::White,
            TextColor::White, 
            TextColor::White,
            TextColor::White,
            TextColor::White
        ];
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
                match colors[i] {
                    TextColor::Green => colors[i] = TextColor::Green,
                    _ => colors[i] = TextColor::Yellow,
                }
                counts[&ch] -= 1;
                if counts[&ch] == 0 {
                    counts.remove(&ch);
                }
            }
        }
        colors
    }
}
