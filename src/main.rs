use std::cmp::Ordering;

use std::io; // allows us to use io lib

use rand::Rng; // makes it so we can use the rand crate

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // stores  a random number between 1-100 into the secret_number variable

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new(); // creates a new mutable string named guess to store user input, it is bound to a new empty instance of string

    io::stdin()
        .read_line(&mut guess) //  We pass &mut guess as a argument telling .read_line what string to store user input in.
        .expect("Failed to read line"); // error handling in case we do not recieve a string

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => println!("You Win!"),
    }
}
