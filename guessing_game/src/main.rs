use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::rng().random_range(1..=100);
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!, the secret number was {secret_number}"),
        Ordering::Greater => println!("Too big!, the secret number was {secret_number}"),
        Ordering::Equal => println!("You win!, the secret number was {secret_number}"),
    }
    println!("Thank you for playing!");
}
