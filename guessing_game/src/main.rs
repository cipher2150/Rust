use std::cmp::Ordering; 
//cmp method compares two values and can be called on anything that can be compared.
use std::io;
use rand::prelude::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100); //random_range(start..=end) inclusive on the lower and upper bounds.

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number){ // cmp return variant of Ordering enum.
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You Win!!"),
    }
}