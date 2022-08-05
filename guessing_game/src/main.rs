use rand::Rng;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret_number is: {secret_number}");

    print!("Input your guess: ");

    io::stdout().flush().unwrap();

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed: {guess}");

    Ok(())
}
