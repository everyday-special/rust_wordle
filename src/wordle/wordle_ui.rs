use std::collections::HashMap;
use std::io;

use super::secret_word::SecretWord;

pub struct WordleUI {
    word: SecretWord,
    letterbank: HashMap<char, String>,
    scores: Vec<Vec<String>>,
}

impl WordleUI {
    pub fn new() -> Self {
        let ascii_lowercase = "abcdefghijklmnopqrstuvwxyz";
        let mut letters = HashMap::new();
        for ch in ascii_lowercase.chars() {
            letters.insert(ch, String::from("white"));
        }
        Self {
            word: SecretWord::new(),
            scores: vec![],
            letterbank: letters,
        }
    }

    fn reset(&mut self) {
        self.word = SecretWord::new();
        let ascii_lowercase = "abcdefghijklmnopqrstuvwxyz";
        let mut letters = HashMap::new();
        for ch in ascii_lowercase.chars() {
            letters.insert(ch, String::from("white"));
        }
        self.letterbank = letters;
    }

    fn get_guess() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        input.strip_suffix("\n").unwrap().to_string()
    }
}