use colored::*;
use std::io;

mod wordle;

use wordle::secret_word::SecretWord;
use wordle::secret_word::TextColor;

fn get_guess() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    input.strip_suffix("\n").unwrap().to_string()
}

fn main() {
    let sw = SecretWord::new();
    sw.print_word();
    let guess = get_guess();
    println!("{}", guess);
    let colors = sw.check_guess(&guess);
    for (i, col) in colors.iter().enumerate() {
        match col {
            TextColor::Green => println!("{}", guess.chars().nth(i).unwrap().to_string().green()),
            TextColor::Yellow => println!("{}", guess.chars().nth(i).unwrap().to_string().yellow()),
            TextColor::White => println!("{}", guess.chars().nth(i).unwrap().to_string().white()),
            TextColor::Grey => println!("{}", guess.chars().nth(i).unwrap().to_string().bold())
,        }
    }
    println!("Allowed: {}", sw.word_allowed(&guess));
    println!("Not allowed: {}", sw.word_allowed(&guess));
}
