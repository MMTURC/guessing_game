use std::cmp::Ordering;

use std::io; // allows us to use io lib

use rand::Rng; // makes it so we can use the rand crate

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100); // stores  a random number between 1-100 into the secret_number variable

    loop {
        println!("Guess the number!");

        println!("Please input your guess.");

        let mut guess = String::new(); // creates a new mutable string named guess to store user input, it is bound to a new empty instance of string

        io::stdin()
            .read_line(&mut guess) //  We pass &mut guess as a argument telling .read_line what string to store user input in.
            .expect("Failed to read line"); // error handling in case we do not recieve a string

        // Here we ignore guessing a non number rather than crashing the program.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // if .parse is able to turn our string into a number then it will return ok

            Err(_) => continue, // if .parse fails we use the _ as a catch all value to match all Err values so it will just execute continue
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}

// Overall we learned about let, match, and different functions along with using external crates like rand.
// Still need to read up a little on match and the whole arms thing but it makes sense
