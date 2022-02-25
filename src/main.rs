mod wordle;

use wordle::wordle_ui::WordleUI;

fn main() {
    let mut ui = WordleUI::new();
    ui.main();
}
