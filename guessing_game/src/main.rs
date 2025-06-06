use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    loop {
        println!("Please input your guess (1-100):");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };

        let secret_number: u32 = rand::rng().random_range(1..=100);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!, the secret number was {secret_number}"),
            Ordering::Greater => println!("Too big!, the secret number was {secret_number}"),
            Ordering::Equal => {
                println!("You win!, the secret number was {secret_number}");
                break;
            }
        }
    }
}
