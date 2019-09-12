use std::io;

pub fn guessing_game() {
    println!("Hey! Guess the number or die :|");
    println!("Please input your guess poor human:");

    let mut guess = String::new();

    let stdin = io::stdin();
    stdin.read_line(&mut guess)
        .expect("I can't read this...");

    println!("You guessed: {}", guess);
}