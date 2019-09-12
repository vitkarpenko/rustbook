use std::cmp::Ordering;
use std::io;

use rand::Rng;

pub fn guessing_game() {
    let secret_number = rand::thread_rng().gen_range(1, 11);
    println!("Hey! Guess the number or die :|");

    loop {
        println!("Please input your guess poor human:");

        let mut guess = String::new();

        let stdin = io::stdin();
        stdin.read_line(&mut guess)
            .expect("I can't read this...");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Nope. Too small, that's what she said."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Bruh that's too much..."),
        };
    }
}