use std::io::{self, Write};

mod game;
use console::Term;
use game::play_game;

fn main() {
    let term = Term::stdout();
    println!("Guess the number!");

    loop {
        term.clear_screen().unwrap();

        play_game();
        print!("Do you want to play again? (y/n) ");
        io::stdout().flush().unwrap();

        let mut play_again = String::new();
        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");

        if play_again.trim().to_lowercase() != "y" {
            term.clear_screen().unwrap();
            break;
        }
    }
}
