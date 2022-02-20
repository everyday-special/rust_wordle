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
            self.print_guesses();
            println!("Invalid word. Please enter a valid five letter word: ");
            guess = self.get_input();
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
        self.print_letterbank();
        println!("");
    }

    fn update_letterbank(&mut self, guess: &String, colors: &Vec<TextColor>) {
        for (i, ch) in guess.chars().enumerate() {
            match self.letterbank[&ch] {
                TextColor::Green => self.letterbank.insert(ch, TextColor::Green),
                TextColor::Yellow => {
                    match colors[i] {
                        TextColor::Green => self.letterbank.insert(ch, TextColor::Green),
                        _ => self.letterbank.insert(ch, TextColor::Yellow),
                    }
                }
                TextColor::White => {
                    match colors[i] {
                        TextColor::White => self.letterbank.insert(ch, TextColor::Black),
                        TextColor::Green => self.letterbank.insert(ch, TextColor::Green),
                        TextColor::Yellow => self.letterbank.insert(ch, TextColor::Yellow),
                        TextColor::Black => self.letterbank.insert(ch, TextColor::Black),
                    }

                }
                TextColor::Black => self.letterbank.insert(ch, TextColor::Black),
            };
        }
    }

    fn print_letterbank(&self) {
        let ascii_lowercase = "abcdefghijklmnopqrstuvwxyz";
        for ch in ascii_lowercase.chars() {
            match self.letterbank[&ch] {
                TextColor::Green => print!("{} ", ch.to_string().green()),
                TextColor::Yellow => print!("{} ", ch.to_string().yellow()),
                TextColor::White => print!("{} ", ch.to_string().white()),
                TextColor::Black => print!("{} ", ch.to_string().black()),
            }
        }
    }
    
    pub fn main(&mut self) {
        let mut score = 0;
        let mut win = false;
        print!("\x1B[2J\x1B[1;1H");
        self.print_letterbank();
        println!("");
        while !win && score < 6 {
            score += 1;
            let guess = self.get_guess();
            let colors = self.word.check_guess(&guess);
            win = self.word.is_exact_match(&guess);
            self.update_letterbank(&guess, &colors);
            let _ = &self.scores.push((guess, colors));
            self.print_guesses();
        }
        if win {
            println!("You win! {}/6", score);
        }
        else {
            print!("You lose! The word was: ");
            self.word.print_word();
        }
    }
}