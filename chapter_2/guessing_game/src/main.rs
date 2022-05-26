use rand::Rng; // defines methods that random number generators implement
use std::cmp::Ordering;
use std::io; //io - input/output library

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");
    // mut makes the variable mutable.
    // The :: syntax indicates that new is an associated function of String
    let mut guess = String::new();
    io::stdin()
        // .read_line() method of the stdin library
        // The & indicates that this argument is a reference
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    // NOTE: crate is a collection of Rust source code files

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
