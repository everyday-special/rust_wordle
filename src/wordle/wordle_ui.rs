use std::collections::HashMap;
use std::io;
use colored::*;

use super::secret_word::SecretWord;
use super::secret_word::TextColor;

pub struct WordleUI {
    word: SecretWord,
    letterbank: HashMap<char, TextColor>,
    scores: Vec<(String, Vec<TextColor>)>,
}

impl WordleUI {
    pub fn new() -> Self {
        let ascii_lowercase = "abcdefghijklmnopqrstuvwxyz";
        let mut letters = HashMap::new();
        for ch in ascii_lowercase.chars() {
            letters.insert(ch, TextColor::White);
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
            letters.insert(ch, TextColor::White);
        }
        self.letterbank = letters;
    }

    fn get_input(&self) -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error: unable to read user input");
        input.strip_suffix("\n").unwrap().to_string()
    }

    fn get_guess(&self) -> String {
        println!("Please enter a guess: ");
        let mut guess = self.get_input();
        while !self.word.word_allowed(&guess) {
            println!("Invalid word. Please enter a valid five letter word: ");
            guess = self.get_input();
            println!("");
        }
        guess
    }

    fn print_guesses(&self) {
        print!("\x1B[2J\x1B[1;1H");
        for (guess, colors) in &self.scores {
            for (i, color) in colors.iter().enumerate() {
                match color {
                    TextColor::Green => print!("{}", guess.chars().nth(i).unwrap().to_string().green()),
                    TextColor::Yellow => print!("{}", guess.chars().nth(i).unwrap().to_string().yellow()),
                    _ => print!("{}", guess.chars().nth(i).unwrap().to_string().white()),
                }
            }
            println!("");
        }
        println!("");
    }
    
    pub fn main(&mut self) {
        let mut _score = 0;
        let _win = false;
        print!("\x1B[2J\x1B[1;1H");
        for _round in 1..7 {
            let guess = self.get_guess();
            let colors = self.word.check_guess(&guess);
            &self.scores.push((guess, colors));
            self.print_guesses();
        }
        println!("End");
    }
}