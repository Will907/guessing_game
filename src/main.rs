use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::process::exit;

fn main() {
    println!("Number guessing game");
    loop {
        match game() {
            true => continue,
            false => exit(0),
        }
    }
}

fn game() -> bool {
    let secret = rand::thread_rng().gen_range(1..=100);
    let mut again = String::new();

    loop {
        let mut guess = String::new();
        println!("Guess a number from 1-100: ");
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

    println!("Again? (y/n)");
    io::stdin()
        .read_line(&mut again)
        .expect("Failed to read line");
    match again.trim().to_lowercase().starts_with("y") {
        true => return true,
        false => return false,
    
    }

}
