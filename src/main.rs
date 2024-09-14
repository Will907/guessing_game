use std::io;
use rand::Rng;

fn main() {
    println!("Number guessing game");
    println!("Guess a number from 1-100: ");
    let mut guess = String::new();
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret}");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");

}
