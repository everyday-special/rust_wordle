mod wordle;

use wordle::wordle_ui::WordleUI;

fn main() {
    /*let sw = SecretWord::new();
    sw.print_word();
    let guess = get_guess();
    println!("{}", guess);
    let colors = sw.check_guess(&guess);
    for (i, col) in colors.iter().enumerate() {
        match col {
            TextColor::Green => print!("{}", guess.chars().nth(i).unwrap().to_string().green()),
            TextColor::Yellow => print!("{}", guess.chars().nth(i).unwrap().to_string().yellow()),
            TextColor::White => print!("{}", guess.chars().nth(i).unwrap().to_string().white()),
            TextColor::Black => print!("{}", guess.chars().nth(i).unwrap().to_string().black()),
        }
    }
    println!("");
    println!("Allowed: {}", sw.word_allowed(&guess));
    println!("Not allowed: {}", sw.word_allowed(&guess));*/
    let mut ui = WordleUI::new();
    ui.main();
}
