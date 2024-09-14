use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Number guessing game");
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret}");

    loop {
        println!("Guess a number from 1-100: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You guessed it!");
                break;
            },
            Ordering::Greater => println!("Too big"),
        }
    }
}
