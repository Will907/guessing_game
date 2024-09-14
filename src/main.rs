use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::process::exit;

fn main() {
    println!("Number guessing game");
    //runs game function until the player specifies to not continue
    loop {
        match game() {
            true => continue,
            false => exit(0),
        }
    }
}

fn game() -> bool {
    //initialises secret number and various miscellaneous variables
    let secret = rand::thread_rng().gen_range(1..=100);
    let mut again = String::new();
    let mut score = 0;

    loop {
        //takes player input
        let mut guess = String::new();
        println!("Guess a number from 1-100: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        //parses input as number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                score += 1;
                num
            },
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        //checks guess against secret number
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                match score {
                    1 => println!("You guessed it in {score} guess!"),
                    _ => println!("You guessed it in {score} guesses!"),
                };
                break;
            },
            Ordering::Greater => println!("Too big"),
        }
    }

    //asks to play again, returns bool value to be processed by main()
    println!("Again? (y/n)");
    io::stdin()
        .read_line(&mut again)
        .expect("Failed to read line");
    match again.trim().to_lowercase().starts_with("y") {
        true => return true,
        false => return false,
    }
}
