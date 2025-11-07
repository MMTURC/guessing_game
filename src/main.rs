use std::io; // allows us to use io lib

fn main() {
    // for reference println! is a macro that prints a string to the screen
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // creates a new mutable string named guess to store user input, it is bound to a new empty instance of string

    io::stdin()
        .read_line(&mut guess) //  We pass &mut guess as a argument telling .read_line what string to store user input in.
        .expect("Failed to read line"); // error handling in case we do not recieve a string

    println!("You guessed: {guess}")
}

// read_line returns a Result Value
