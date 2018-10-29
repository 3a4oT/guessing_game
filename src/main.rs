extern crate rand;
extern crate ansi_term;

use rand::Rng;
use ansi_term::Colour::Red;
use ansi_term::Color::Blue;
use ansi_term::Color::Green;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");
    let secret_number: u32 = rand::thread_rng().gen_range(1, 100);
    println!("The secret number is {}", secret_number);
    begin_game(secret_number);
}

fn begin_game(secret_number: u32) {
    loop {
        println!("{}", Blue.paint("Please input your guess."));
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", Red.paint("Too small!")),
            Ordering::Greater => println!("{}", Red.paint("Too big!")),
            Ordering::Equal => {
                println!("{}", Green.paint("You win!"));
                break;
            }
        }
    }
}
