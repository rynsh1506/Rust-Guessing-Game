use console::Term;
use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn get_guess() -> u8 {
    let term = Term::stdout();
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    match guess.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            term.clear_screen().unwrap();
            print!("Please type a number: ");
            io::stdout().flush().unwrap();
            get_guess()
        }
    }
}

pub fn play_game() {
    let secret_number: u8 = rand::thread_rng().gen_range(1..=10);
    let mut life: u8 = 3;
    let term = Term::stdout();

    loop {
        if life == 0 {
            println!("You ran out of lives! Game over.");
            println!("The secret number is: {}", secret_number);
            break;
        }

        print!("Please enter your number: ");
        io::stdout().flush().unwrap();

        let guess = get_guess();

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                term.clear_screen().unwrap();
                println!("Too small");
                life -= 1;
            }
            Ordering::Greater => {
                term.clear_screen().unwrap();
                println!("Too big");
                life -= 1;
            }
            Ordering::Equal => {
                term.clear_screen().unwrap();
                println!("You win! you guessed: {}", guess);
                println!("The secret number is: {}", true);
                break;
            }
        }
    }
}
